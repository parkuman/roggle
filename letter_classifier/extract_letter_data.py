# extract_letter_data.py
# Author: Parker Rowe
# Date: June 2022
# 
# This notebook is responsible for taking the labelled boggle dataset from Roboflow,
# and cropping each letter from the image given the bounding box annotion found 
# in the annotation csv file. It then saves each cropped image to their respective
# folder in the test/train dataset folders. 

import cv2
import os


folder = "nightTrain"
annotations_path = f"./raw_data/BoggleBoards.v4-416x416-auto-orient.tensorflow/export/_annotations.csv"
images_path = f"./raw_data/BoggleBoards.v4-416x416-auto-orient.tensorflow/export/"
output_path =  "./train/"

annotations = open(annotations_path)
annotations.readline() # skip header
line = annotations.readline() # read first line

while line:
  data = (line.strip()).split(",")

  classification = data[3]

  if classification == "in" or classification == "th" or classification == "an" or classification == "er" or classification == "he" or classification == "wild":
    # skip the letter blocks we don't support
    line = annotations.readline() 
    continue

  file_name = data[0]
  original_file_path = os.path.join(images_path, file_name)
  new_file_path = os.path.join(output_path, classification + "/", file_name)

  image = cv2.imread(original_file_path)
  (x_min, y_min, x_max, y_max) = list(map(int, data[4:8])) 
  
  cropped_img = image[y_min:y_max, x_min:x_max]

  cropped_img = cv2.cvtColor(cropped_img, cv2.COLOR_BGR2GRAY)
  cv2.imwrite(new_file_path, cropped_img)

  line = annotations.readline() 
  # break

annotations.close()

print("Done!")