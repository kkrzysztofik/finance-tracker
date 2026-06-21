use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "INSERT INTO categories (name, name_pl) VALUES
                    ('Groceries', 'Artykuły spożywcze'),
                    ('Restaurants & Cafes', 'Restauracje i kawiarnie'),
                    ('Transport', 'Transport'),
                    ('Fuel', 'Paliwo'),
                    ('Housing & Rent', 'Mieszkanie i czynsz'),
                    ('Utilities', 'Opłaty i media'),
                    ('Healthcare', 'Zdrowie'),
                    ('Entertainment', 'Rozrywka'),
                    ('Shopping', 'Zakupy'),
                    ('Clothing', 'Odzież'),
                    ('Electronics', 'Elektronika'),
                    ('Education', 'Edukacja'),
                    ('Subscriptions', 'Subskrypcje'),
                    ('Insurance', 'Ubezpieczenia'),
                    ('Transfers', 'Przelewy'),
                    ('ATM Withdrawal', 'Wypłata z bankomatu'),
                    ('Salary', 'Wynagrodzenie'),
                    ('Freelance Income', 'Dochód z freelancingu'),
                    ('Pubs & Bars', 'Puby i bary'),
                    ('Fitness & Sport', 'Fitness i sport'),
                    ('Personal Care', 'Higiena osobista'),
                    ('Pets', 'Zwierzęta'),
                    ('Gifts', 'Prezenty'),
                    ('Travel', 'Podróże'),
                    ('Other', 'Inne')
                 ON CONFLICT (name) DO NOTHING;",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared(
                "DELETE FROM categories WHERE name IN (
                    'Groceries',
                    'Restaurants & Cafes',
                    'Transport',
                    'Fuel',
                    'Housing & Rent',
                    'Utilities',
                    'Healthcare',
                    'Entertainment',
                    'Shopping',
                    'Clothing',
                    'Electronics',
                    'Education',
                    'Subscriptions',
                    'Insurance',
                    'Transfers',
                    'ATM Withdrawal',
                    'Salary',
                    'Freelance Income',
                    'Pubs & Bars',
                    'Fitness & Sport',
                    'Personal Care',
                    'Pets',
                    'Gifts',
                    'Travel',
                    'Other'
                );",
            )
            .await?;

        Ok(())
    }
}
