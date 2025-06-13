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
for table in wait.until(EC.presence_of_all_elements_located((By.CSS_SELECTOR, "table tr")))[1:]:
    data = [item.text for item in table.find_elements(By.CSS_SELECTOR, "th, td")]
    print(json.dumps(data))
    print(data)
driver.quit()