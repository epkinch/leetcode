SELECT d.name AS Department, e.name AS Employee, salary AS Salary
FROM Employee e 
    JOIN Department d ON departmentId = d.id
WHERE salary = (
    SELECT MAX(salary)
    FROM Employee
    WHERE departmentId = d.id);