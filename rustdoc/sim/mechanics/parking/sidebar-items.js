initSidebarItems({"enum":[["ParkingSimState",""]],"struct":[["InfiniteParkingSimState","This assigns infinite private parking to all buildings and none anywhere else. This effectively disables the simulation of parking entirely, making driving trips just go directly between buildings. Useful for maps without good parking data (which is currently all of them) and experiments where parking contention skews results and just gets in the way."],["NormalParkingSimState",""],["ParkingLane",""]],"trait":[["ParkingSim","Manages the state of parked cars. There are two implementations: - NormalParkingSimState allows only one vehicle per ParkingSpot defined in the map - InfiniteParkingSimState pretends every building has infinite capacity, and onstreet parking is   ignored"]]});