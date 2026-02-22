import sys
import time
import random
import logging

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

try:
    import edge_ids
    ids_engine = edge_ids.EdgeIDS()
except (ImportError, RuntimeError) as e:
    logging.warning(f"Native module error: {e}. Falling back to Robust Mock.")
    class RobustMockIDS:
        def run_inference(self, data):
            if not data:
                raise ValueError("Python side: Data is empty!")
            return random.random() > 0.98

    ids_engine = RobustMockIDS()

def real_time_stream():
    """
    Simulates a real-time data stream from a network interface
    """
    logging.info("Starting Real-Time Edge Data Stream...")
    try:
        while True:
            # Simulate a 1500-byte MTU packet
            packet = [random.randint(0, 255) for _ in range(1500)]
            
            try:
                # Call C++ engine with exception handling
                detection = ids_engine.run_inference(packet)
                if detection:
                    logging.info(f"[DETECTION] Threat signature identified in packet of size {len(packet)}")
            except Exception as e:
                logging.error(f"Engine processing error: {e}")
            
            # High frequency simulation
            time.sleep(0.1) 
    except KeyboardInterrupt:
        logging.info("Stream terminated by user.")

if __name__ == "__main__":
    real_time_stream()
