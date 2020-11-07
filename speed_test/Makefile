all:
	cargo test -- --nocapture

benchmark_mutex:
	cargo test benchmark_mutex  -- --nocapture

benchmark_rwlock:
	cargo test benchmark_rwlock_write  -- --nocapture \
	&& cargo test benchmark_rwlock_read -- --nocapture

benchmark_parking_lot_rwlock:
	cargo test benchmark_lot_rwlock_write benchmark_lot_rwlock_read  -- --nocapture