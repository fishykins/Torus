import os
from pathlib import Path
dir_path = os.path.dirname(os.path.realpath(__file__))

plugin_name = "utor"
extention = "so" #TODO: Add windows support. 

root = Path(dir_path).parent
station_gen_path = os.path.join(root, "Utor")

plugin_path = os.path.join(root, "Game/Assets/Utor/lib" + plugin_name + "." + extention)
build_path = os.path.join(root, "target/debug/lib" + plugin_name + "." + extention)

os.chdir(station_gen_path)
os.system("cargo build")
# move the release into unity project
os.system("mv " + build_path + " " + plugin_path)