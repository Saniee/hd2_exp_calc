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

# Parse tables.
# TODO: Save the data into a json file with the following scheme.
"""
{
    "Level": number,
    "total_experience": number,
    "required_experience_for_next": number
}
"""
# Variable to store data from the site for further processing
data = []

for table in wait.until(EC.presence_of_all_elements_located((By.CSS_SELECTOR, "table tr")))[1:]:
    # TODO: Rename idk to something that makes sense
    idk = [item.text for item in table.find_elements(By.CSS_SELECTOR, "th, td")]
    data.append(idk)
    
driver.quit()

variable = {}

for item in data:
    
    if item[0].isnumeric() == True:
        variable[f"Level {item[0]}"] = {"level": item[0], "total_expirience": item[1], "required_experience_for_next": item[2]}
    elif item[2].isnumeric() == True:
        variable[f"Level {item[2]}"] = {"level": item[2], "total_expirience": item[3], "required_experience_for_next": item[4]}
    else:
        continue
    
print(variable)

# Opening and writing the data we want into a json file, if the json file doesn't exist, it will be created
with open("xp_table.json", encoding="utf-8", mode="w") as f:
    f.write(json.dumps(variable))