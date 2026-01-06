# Collez le script ici...
# /!\ SUPPRIMEZ LA LIGNE "!pip install roboflow"

from roboflow import Roboflow
rf = Roboflow(api_key="BgxDSfNWlg2O3oLBHEZz")
project = rf.workspace("sffs-viro0").project("my-first-project-lxirr")
version = project.version(1)
dataset = version.download("yolov12")
                