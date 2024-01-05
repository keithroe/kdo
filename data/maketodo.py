import random
from faker import Faker

fake = Faker()

with open("todo.txt", "w") as file:
    for _ in range(100):
        priority = random.choices(["", "(A)", "(B)", "(C)"], [0.3, 0.4, 0.2, 0.1])[0]
        completion_status = random.choices(["", "x"], [0.3, 0.7])[0]
        creation_date = fake.date_this_decade()
        task_description = fake.sentence()
        context = random.choices(["", "@work", "@home"], [0.2, 0.4, 0.4])[0];
        num_projects = random.choices([0, 1, 2], [0.1, 0.6, 0.3])[0]
        projects = random.choices(["+projA", "+projB", "+projC", "+projD", "+projE"], k=num_projects)



        file.write(f"{completion_status} {priority} {creation_date} {task_description.strip()} {context}".strip())
        for p in projects:
            file.write(f" {p}")
        file.write(f"\n")
