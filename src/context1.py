class Connection:
    def __init__(self, address):
        self.address = address
    
    def __enter__(self):
        print(f"Connection open to {self.address}")
        return 123
    
    def __exit__(self, ex_type, ex_val, ex_tb):
        print("Connection closed")
    
with Connection("target") as conn:
    print("Doing something well")
    
with Connection("target") as conn:
    raise Exception("Doing something wrong")
