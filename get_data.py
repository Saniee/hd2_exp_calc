from selenium.webdriver import Firefox
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.common.by import By
from selenium.webdriver.support import expected_conditions as EC

import json

from time import sleep

driver = Firefox()
driver.get("https://helldivers.wiki.gg/wiki/Ranks")
wait = WebDriverWait(driver, 10)

# Wait till the cookie popup is manually exited out of.
sleep(10)


# Variable to store data from the site for further processing
data = []
# Parsing tables.
for table in wait.until(EC.presence_of_all_elements_located((By.CSS_SELECTOR, "table tr")))[1:-1]:
    elements = [item.text for item in table.find_elements(By.CSS_SELECTOR, "th, td")]
    data.append(elements)

# Firefox not needed anymore.
driver.quit()

# print(data)

# Variable that stores processed data.
table_data = []
# Elements processing starts.
for item in data:
    if item[0].isnumeric() == True:
        variable = {
            "level": int(item[0].replace(",", "")),
            "total_experience": int(item[1].replace(",", "")),
            "required_experience_for_next": int(item[2].replace(",", ""))
        }
        table_data.append(variable)
    elif item[2].isnumeric() == True:
        # Last number is not defined, as it's the max rank.
        if item[4] != "":
            variable = {
                "level": int(item[2].replace(",", "")), 
                "total_experience": int(item[3].replace(",", "")),
                "required_experience_for_next": int(item[4].replace(",", ""))
            }
            table_data.append(variable)
        else:   
            variable = {
                "level": int(item[2].replace(",", "")), 
                "total_experience": int(item[3].replace(",", "")),
                "required_experience_for_next": 0
            }
            table_data.append(variable)
    else:
        continue

# Opening and writing the data we want into a json file, if the json file doesn't exist, it will be created
with open("xp_table.json", encoding="utf-8", mode="w") as f:
    f.write(json.dumps(table_data, sort_keys=True, indent=4))