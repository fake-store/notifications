pub fn notify(user_id: &str, order_id: &str, tracking_number: &str) {
    tracing::info!(
        user_id = user_id,
        order_id = order_id,
        tracking_number = tracking_number,
        "Shipment notification: order shipped, would notify customer"
    );
}
