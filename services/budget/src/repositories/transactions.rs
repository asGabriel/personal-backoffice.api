use crate::domains::{
    errors::Result,
    transactions::{
        CreateTransaction, Transaction, TransactionCategory, TransactionRecurrency,
        TransactionStatus, TransactionType, UpdateTransaction,
    },
};

use super::SqlxRepository;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait TransactionRepository {
    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction>;
    async fn list_transactions(&self) -> Result<Vec<Transaction>>;
    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>>;
    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>>;
}

#[async_trait::async_trait]
impl TransactionRepository for SqlxRepository {
    async fn list_transactions(&self) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    async fn create_transaction(&self, transaction: CreateTransaction) -> Result<Transaction> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            INSERT INTO TRANSACTIONS (
                transaction_id,
                movement_type,
                description,
                amount,
                due_date,
                category,
                account_id,
                recurring,
                recurrence_frequency,
                recurrence_duration_months,
                status
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) RETURNING 
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            Uuid::new_v4(),
            transaction.movement_type as TransactionType,
            transaction.description,
            transaction.amount,
            transaction.due_date,
            transaction.category as TransactionCategory,
            transaction.account_id,
            transaction.recurring,
            transaction.recurrence_frequency as TransactionRecurrency,
            transaction.recurrence_duration_months,
            transaction.status as TransactionStatus
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn get_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            SELECT
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            FROM TRANSACTIONS
            WHERE transaction_id = $1
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn delete_transaction_by_id(&self, transaction_id: Uuid) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                updated_at = now(),
                deleted_at = now()
            WHERE
                transaction_id = $1
                AND deleted_at is null
            RETURNING 
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }

    async fn update_transaction_by_id(
        &self,
        transaction: Transaction,
        payload: UpdateTransaction,
    ) -> Result<Option<Transaction>> {
        let transaction = sqlx::query_as!(
            Transaction,
            r#"
            UPDATE TRANSACTIONS SET
                movement_type = $2,
                description = $3,
                amount = $4,
                due_date = $5,
                category = $6,
                account_id = $7,
                recurring = $8,
                recurrence_frequency = $9,
                recurrence_duration_months = $10,
                note = $11,
                status = $12,
                updated_at = now()
            WHERE 
                transaction_id = $1
            RETURNING
                transaction_id, 
                movement_type as "movement_type!: TransactionType",
                description, 
                amount, 
                due_date, 
                category as "category: TransactionCategory", 
                account_id, 
                recurring, 
                recurrence_frequency as "recurrence_frequency: TransactionRecurrency", 
                recurrence_duration_months, 
                status as "status: TransactionStatus", 
                note, 
                created_at, 
                updated_at, 
                deleted_at
            "#,
            transaction.transaction_id,
            payload.movement_type.unwrap_or(transaction.movement_type) as TransactionType,
            payload.description.unwrap_or(transaction.description),
            payload.amount.unwrap_or(transaction.amount),
            payload.due_date.unwrap_or(transaction.due_date),
            payload.category.unwrap_or(transaction.category) as TransactionCategory,
            payload.account_id.unwrap_or(transaction.account_id),
            payload.recurring.unwrap_or(transaction.recurring),
            payload
                .recurrence_frequency
                .unwrap_or(transaction.recurrence_frequency) as TransactionRecurrency,
            payload
                .recurrence_duration_months
                .unwrap_or(transaction.recurrence_duration_months),
            payload.note.unwrap_or(transaction.note),
            payload.status.unwrap_or(transaction.status) as TransactionStatus
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(transaction)
    }
}
