export interface HeartbeatCache {
    elapsed: {
        secs: number,
        nanos: number
    };
    send_count: number;
    packet_continuous_loss_count: number;
    packet_continuous_recv_count: number;
    packet_loss_count: number;
}