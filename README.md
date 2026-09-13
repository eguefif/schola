# Schola

Local First app for teacher. Practice project to learn Tauri/Vue

# TODO

## Next

- [ ] Add table curriculum
- [ ] Add API call to TeacherCoop to retrieve Curriculum
- [ ] Add key store for year information
- [ ] Add API call to TeacherCoop to retrieve year

## Database

- [x] Add Sqlite store with Tauri


## Year Plan

- [ ] Add a plan table: name, grade
- [ ] Add a join table plan/curriculum named period: this should have a period_number
- [ ] Modify plan/indexes to add a new plan button
- [ ] Add a simple form to add a new plan:


 ## Data base represerntation

We have three tables:
- curriculum
- school_period (join table between plan and curriculum that indicates what curriculum item is related to what plan for what period)
- plan
