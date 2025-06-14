// services/realtime-gateway/src/spatial_streaming.rs

pub struct SpatialStreamManager {
    player_positions: DashMap<PlayerId, Position3D>,
    grid_subscribers: DashMap<GridCoordinate, HashSet<PlayerId>>,
    object_cache: ObjectCache,
}

impl SpatialStreamManager {
    pub async fn handle_player_movement(
        &self,
        player_id: PlayerId,
        new_position: Position3D,
    ) -> StreamUpdate {
        let old_grids = self.get_visible_grids(
            self.player_positions.get(&player_id).map(|p| *p)
        );
        let new_grids = self.get_visible_grids(Some(new_position));

        // Calculate grid transitions
        let grids_to_load = new_grids.difference(&old_grids);
        let grids_to_unload = old_grids.difference(&new_grids);

        // Update subscriptions
        self.update_grid_subscriptions(player_id, &new_grids).await;

        StreamUpdate {
            load_grids: self.get_grid_data(grids_to_load).await,
            unload_grids: grids_to_unload.cloned().collect(),
            nearby_entities: self.get_nearby_entities(new_position).await,
            lod_updates: self.calculate_lod_changes(new_position).await,
        }
    }

    fn get_visible_grids(&self, position: Option<Position3D>) -> HashSet<GridCoordinate> {
        // Return grids within view distance (typically 3x3 grid area)
        // ...
    }
}