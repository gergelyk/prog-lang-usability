# DB in a container:
# docker run --name productsdb -e POSTGRES_PASSWORD=pass -p 5432:5432 -d postgres:15.3


import sqlalchemy as db
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column
from sqlalchemy.orm import MappedAsDataclass, Session

class Base(MappedAsDataclass, DeclarativeBase):
#    metadata = db.MetaData()
    pass

class StockItem(Base):
    __tablename__ = "stock"
    name: Mapped[str] = mapped_column(primary_key=True)
    vendor: Mapped[str]
    quantity: Mapped[int]

engine = db.create_engine("postgresql+psycopg2://postgres:pass@127.0.0.1/postgres",)
#Base.metadata.create_all(engine)

with Session(engine) as session:
    session.add_all([
        StockItem(name="Toothbrush", vendor="Limo", quantity=1497),
        StockItem(name="Comb", vendor="Takoon", quantity=210),
        StockItem(name="Towel", vendor="Beana", quantity=362),
        ])

    session.commit()    

    query = db.select(StockItem).where(StockItem.quantity >= 300)
    for user in session.scalars(query):
        print(user)


