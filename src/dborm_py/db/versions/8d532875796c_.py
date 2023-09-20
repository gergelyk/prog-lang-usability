"""empty message

Revision ID: 8d532875796c
Revises: 
Create Date: 2023-07-23 19:17:13.880005

"""
from alembic import op
import sqlalchemy as sa


# revision identifiers, used by Alembic.
revision = '8d532875796c'
down_revision = None
branch_labels = None
depends_on = None


def upgrade():
    op.create_table(
        'stock',
        sa.Column('name', sa.String, primary_key=True),
        sa.Column('vendor', sa.String),
        sa.Column('quantity', sa.Integer),
    )

def downgrade():
    op.drop_table('stock')
