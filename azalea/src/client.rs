/// `Client` has the things that a user interacting with the library will want.
/// Things that a player in the world will want to know are in [`LocalPlayer`].
///
/// To make a new client, use either [`azalea::ClientBuilder`] or
/// [`Client::join`].
///
/// [`azalea::ClientBuilder`]: https://docs.rs/azalea/latest/azalea/struct.ClientBuilder.html
#[derive(Clone)]
pub struct Client {
    /// The [`GameProfile`] for our client. This contains your username, UUID,
    /// and skin data.
    ///
    /// This is immutable; the server cannot change it. To get the username and
    /// skin the server chose for you, get your player from
    /// [`Self::players`].
    pub profile: GameProfile,
    /// The entity for this client in the ECS.
    pub entity: Entity,
    /// The world that this client is in.
    pub world: Arc<RwLock<PartialWorld>>,

    /// The entity component system. You probably don't need to access this
    /// directly. Note that if you're using a shared world (i.e. a swarm), this
    /// will contain all entities in all worlds.
    pub ecs: Arc<Mutex<Ecs>>,

    /// Use this to force the client to run the schedule outside of a tick.
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

impl Client {
    /// Create a new client from the given GameProfile, Connection, and World.
    /// You should only use this if you want to change these fields from the
    /// defaults, otherwise use [`Client::join`].
    pub fn new(
        profile: GameProfile,
        entity: Entity,
        ecs: Arc<Mutex<Ecs>>,
        run_schedule_sender: mpsc::UnboundedSender<()>,
    ) -> Self {
        Self {
            profile,
            // default our id to 0, it'll be set later
            entity,
            world: Arc::new(RwLock::new(PartialWorld::default())),

            ecs,

            run_schedule_sender,
        }
    }

    /// Write a packet directly to the server.
    pub fn write_packet(&self, packet: ServerboundGamePacket) {
        self.local_player_mut(&mut self.ecs.lock())
            .write_packet(packet);
    }

    /// Disconnect this client from the server by ending all tasks.
    ///
    /// The OwnedReadHalf for the TCP connection is in one of the tasks, so it
    /// automatically closes the connection when that's dropped.
    pub fn disconnect(&self) {
        self.ecs.lock().send_event(DisconnectEvent {
            entity: self.entity,
        });
    }

    pub fn local_player<'a>(&'a self, ecs: &'a mut Ecs) -> &'a LocalPlayer {
        self.query::<&LocalPlayer>(ecs)
    }
    pub fn local_player_mut<'a>(
        &'a self,
        ecs: &'a mut Ecs,
    ) -> azalea_ecs::ecs::Mut<'a, LocalPlayer> {
        self.query::<&mut LocalPlayer>(ecs)
    }

    /// Get a component from this client. This will clone the component and
    /// return it.
    ///
    /// # Panics
    ///
    /// This will panic if the component doesn't exist on the client.
    ///
    /// # Examples
    ///
    /// ```
    /// # use azalea_world::entity::WorldName;
    /// # fn example(client: &azalea_client::Client) {
    /// let world_name = client.component::<WorldName>();
    /// # }
    pub fn component<T: Component + Clone>(&self) -> T {
        self.query::<&T>(&mut self.ecs.lock()).clone()
    }

    /// Get a reference to our (potentially shared) world.
    ///
    /// This gets the [`World`] from our world container. If it's a normal
    /// client, then it'll be the same as the world the client has loaded.
    /// If the client using a shared world, then the shared world will be a
    /// superset of the client's world.
    pub fn world(&self) -> Arc<RwLock<World>> {
        let world_name = self.component::<WorldName>();
        let ecs = self.ecs.lock();
        let world_container = ecs.resource::<WorldContainer>();
        world_container.get(&world_name).unwrap()
    }

    /// Returns whether we have a received the login packet yet.
    pub fn logged_in(&self) -> bool {
        // the login packet tells us the world name
        self.query::<Option<&WorldName>>(&mut self.ecs.lock())
            .is_some()
    }

    /// Tell the server we changed our game options (i.e. render distance, main
    /// hand). If this is not set before the login packet, the default will
    /// be sent.
    ///
    /// ```rust,no_run
    /// # use azalea_client::{Client, ClientInformation};
    /// # async fn example(bot: Client) -> Result<(), Box<dyn std::error::Error>> {
    /// bot.set_client_information(ClientInformation {
    ///     view_distance: 2,
    ///     ..Default::default()
    /// })
    /// .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_client_information(
        &self,
        client_information: ServerboundClientInformationPacket,
    ) -> Result<(), std::io::Error> {
        {
            self.local_player_mut(&mut self.ecs.lock())
                .client_information = client_information;
        }

        if self.logged_in() {
            let client_information_packet = self
                .local_player(&mut self.ecs.lock())
                .client_information
                .clone()
                .get();
            log::debug!(
                "Sending client information (already logged in): {:?}",
                client_information_packet
            );
            self.write_packet(client_information_packet);
        }

        Ok(())
    }

    /// Get a HashMap of all the players in the tab list.
    pub fn players(&mut self) -> HashMap<Uuid, PlayerInfo> {
        self.local_player(&mut self.ecs.lock()).players.clone()
    }
}
