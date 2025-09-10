# Finance Backend API Documentation

**Base URL**: `http://127.0.0.1:8080/api`

---

## 🔐 Authentication Endpoints

### 1. User Registration
- **Method**: `POST`
- **URL**: `/api/auth/register`
- **Body** (JSON):
```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "password": "password123"
}
```
- **Response**:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2025-09-10T10:00:00Z"
  }
}
```

### 2. User Login
- **Method**: `POST`
- **URL**: `/api/auth/login`
- **Body** (JSON):
```json
{
  "email": "john@example.com",
  "password": "password123"
}
```
- **Response**: Same as registration
- **Note**: Requires OTP verification before login

### 3. Request OTP Code
- **Method**: `POST`
- **URL**: `/api/auth/request-otp`
- **Body** (JSON):
```json
{
  "email": "john@example.com"
}
```
- **Response**:
```json
{
  "ok": true
}
```
- **Note**: Sends OTP code via email (if SMTP configured) or logs to console

### 4. Verify OTP Code
- **Method**: `POST`
- **URL**: `/api/auth/verify-otp`
- **Body** (JSON):
```json
{
  "email": "john@example.com",
  "code": "123456"
}
```
- **Response**:
```json
{
  "verified": true
}
```
- **Note**: Must verify OTP before login

### 5. Google OAuth Login
- **Method**: `POST`
- **URL**: `/api/auth/google`
- **Body** (JSON):
```json
{
  "id_token": "google_id_token_here"
}
```
- **Response**: Same as user login
- **Note**: Requires GOOGLE_CLIENT_ID configured and OTP verification

### 6. Forgot Password
- **Method**: `POST`
- **URL**: `/api/auth/forgot-password`
- **Body** (JSON):
```json
{
  "email": "john@example.com"
}
```
- **Response**:
```json
{
  "ok": true
}
```
- **Note**: Sends password reset OTP code

### 7. Reset Password
- **Method**: `POST`
- **URL**: `/api/auth/reset-password`
- **Body** (JSON):
```json
{
  "email": "john@example.com",
  "code": "123456",
  "new_password": "newpassword123"
}
```
- **Response**:
```json
{
  "reset": true
}
```

### 8. Logout
- **Method**: `POST`
- **URL**: `/api/auth/logout`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**:
```json
{
  "ok": true
}
```

### 9. Get Current User Profile
- **Method**: `GET`
- **URL**: `/api/me`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2025-09-10T10:00:00Z"
}
```

### 10. Delete Current User Account
- **Method**: `DELETE`
- **URL**: `/api/me`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**: `204 No Content`
- **Note**: Permanently deletes user account and all associated data

---

## 📂 Category Endpoints

### 11. List Categories
- **Method**: `GET`
- **URL**: `/api/categories`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**:
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Salary",
    "kind": "income",
    "color": "#00ff00",
    "created_at": "2025-09-10T10:00:00Z"
  }
]
```

### 12. Create Category
- **Method**: `POST`
- **URL**: `/api/categories`
- **Headers**: `Authorization: Bearer <user_token>`
- **Body** (JSON):
```json
{
  "name": "Food",
  "kind": "expense",
  "color": "#ff0000"
}
```
- **Note**: `kind` must be either "income" or "expense", `color` is optional (defaults to "#888888")

### 13. Update Category
- **Method**: `PUT`
- **URL**: `/api/categories/{category_id}`
- **Headers**: `Authorization: Bearer <user_token>`
- **Body** (JSON):
```json
{
  "name": "Groceries",
  "kind": "expense",
  "color": "#ff5555"
}
```
- **Note**: All fields are optional

### 14. Delete Category
- **Method**: `DELETE`
- **URL**: `/api/categories/{category_id}`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**: `204 No Content`

---

## 💰 Transaction Endpoints

### 15. List Transactions
- **Method**: `GET`
- **URL**: `/api/transactions`
- **Headers**: `Authorization: Bearer <user_token>`
- **Query Parameters** (all optional):
  - `category_id`: UUID to filter by category
  - `start_date`: YYYY-MM-DD format
  - `end_date`: YYYY-MM-DD format
- **Example**: `/api/transactions?start_date=2025-09-01&end_date=2025-09-30`
- **Response**:
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440002",
    "user_id": "550e8400-e29b-41d4-a716-446655440000",
    "category_id": "550e8400-e29b-41d4-a716-446655440001",
    "amount": "5000.00",
    "occurred_on": "2025-09-10",
    "description": "Monthly salary"
  }
]
```

### 16. Create Transaction
- **Method**: `POST`
- **URL**: `/api/transactions`
- **Headers**: `Authorization: Bearer <user_token>`
- **Body** (JSON):
```json
{
  "category_id": "550e8400-e29b-41d4-a716-446655440001",
  "amount": "1500.50",
  "occurred_on": "2025-09-10",
  "description": "Grocery shopping"
}
```

### 17. Update Transaction
- **Method**: `PUT`
- **URL**: `/api/transactions/{transaction_id}`
- **Headers**: `Authorization: Bearer <user_token>`
- **Body** (JSON):
```json
{
  "category_id": "550e8400-e29b-41d4-a716-446655440001",
  "amount": "1600.00",
  "occurred_on": "2025-09-10",
  "description": "Updated grocery shopping"
}
```
- **Note**: All fields are optional

### 18. Delete Transaction
- **Method**: `DELETE`
- **URL**: `/api/transactions/{transaction_id}`
- **Headers**: `Authorization: Bearer <user_token>`
- **Response**: `204 No Content`

---

## 📊 Summary Endpoints

### 19. Monthly Summary
- **Method**: `GET`
- **URL**: `/api/summary/month`
- **Headers**: `Authorization: Bearer <user_token>`
- **Query Parameters** (required):
  - `year`: Integer (e.g., 2025)
  - `month`: Integer 1-12
- **Example**: `/api/summary/month?year=2025&month=9`
- **Response**:
```json
{
  "year": 2025,
  "month": 9,
  "total_income": "5000.00",
  "total_expense": "3500.00",
  "balance": "1500.00",
  "category_breakdown": [
    {
      "category_id": "550e8400-e29b-41d4-a716-446655440001",
      "name": "Salary",
      "kind": "income",
      "total": "5000.00"
    },
    {
      "category_id": "550e8400-e29b-41d4-a716-446655440002",
      "name": "Food",
      "kind": "expense",
      "total": "1200.00"
    }
  ]
}
```

---

## 👑 Admin Authentication Endpoints

### 20. Admin Registration
- **Method**: `POST`
- **URL**: `/api/admin/auth/register`
- **Body** (JSON):
```json
{
  "name": "Admin User",
  "email": "admin@example.com",
  "password": "adminpass123"
}
```
- **Note**: First admin can register without authentication. Subsequent admins need existing admin token.
- **Headers** (for subsequent registrations): `Authorization: Bearer <admin_token>`

### 21. Admin Login
- **Method**: `POST`
- **URL**: `/api/admin/auth/login`
- **Body** (JSON):
```json
{
  "email": "admin@example.com",
  "password": "adminpass123"
}
```
- **Response**:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "admin": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Admin User",
    "email": "admin@example.com",
    "created_at": "2025-09-10T10:00:00Z"
  }
}
```

### 22. Get Current Admin Profile
- **Method**: `GET`
- **URL**: `/api/admin/me`
- **Headers**: `Authorization: Bearer <admin_token>`

---

## 👥 Admin User Management Endpoints

### 23. List All Users
- **Method**: `GET`
- **URL**: `/api/admin/users`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Response**:
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "John Doe",
    "email": "john@example.com",
    "created_at": "2025-09-10T10:00:00Z"
  }
]
```

### 24. Get User by ID
- **Method**: `GET`
- **URL**: `/api/admin/users/{user_id}`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Response**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2025-09-10T10:00:00Z"
}
```

### 25. Create User (Admin)
- **Method**: `POST`
- **URL**: `/api/admin/users`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Body** (JSON):
```json
{
  "name": "New User",
  "email": "newuser@example.com",
  "password": "password123"
}
```

### 26. Update User (Admin)
- **Method**: `PUT`
- **URL**: `/api/admin/users/{user_id}`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Body** (JSON):
```json
{
  "name": "Updated Name",
  "email": "updated@example.com",
  "password": "newpassword123"
}
```
- **Note**: All fields are optional

### 27. Delete User (Admin)
- **Method**: `DELETE`
- **URL**: `/api/admin/users/{user_id}`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Response**: `204 No Content`

---

## ⚙️ Admin Settings Endpoints

### 28. List Application Settings
- **Method**: `GET`
- **URL**: `/api/admin/settings`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Response**:
```json
[
  {
    "key": "app_name",
    "value": "Finance Tracker",
    "updated_by": "550e8400-e29b-41d4-a716-446655440000",
    "updated_at": "2025-09-10T10:00:00Z"
  },
  {
    "key": "max_categories_per_user",
    "value": "50",
    "updated_by": "550e8400-e29b-41d4-a716-446655440000",
    "updated_at": "2025-09-10T09:30:00Z"
  }
]
```

### 29. Update/Create Setting
- **Method**: `PUT`
- **URL**: `/api/admin/settings/{setting_key}`
- **Headers**: `Authorization: Bearer <admin_token>`
- **Body** (JSON):
```json
{
  "value": "New Setting Value"
}
```
- **Response**:
```json
{
  "key": "setting_key",
  "value": "New Setting Value",
  "updated_by": "550e8400-e29b-41d4-a716-446655440000",
  "updated_at": "2025-09-10T10:15:00Z"
}
```
- **Note**: This is an upsert operation (creates if doesn't exist, updates if exists)

---

## 🏥 Health Check

### 30. Health Check
- **Method**: `GET`
- **URL**: `/api/healthz`
- **Response**:
```json
{
  "status": "ok"
}
```

---

## 🚨 Error Responses

All endpoints may return these error status codes:
- `400 Bad Request`: Invalid input data
- `401 Unauthorized`: Missing or invalid token  
- `403 Forbidden`: Access denied
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists (e.g., email already registered)
- `500 Internal Server Error`: Server error

Error response format:
```json
{
  "error": "Error message description"
}
```