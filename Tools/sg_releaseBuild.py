import os
from pathlib import Path
dir_path = os.path.dirname(os.path.realpath(__file__))

root = Path(dir_path).parent
station_gen_path = os.path.join(root, "station_gen")
build_path = os.path.join(root, "torus/Assets/station_gen")
os.chdir(station_gen_path)

os.system("cargo build --target-dir " + build_path)
#os.system("cargo build --release --target-dir " + build_dir)