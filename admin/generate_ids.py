#!/usr/bin/env python3
from uuid import uuid4
from uuid import UUID
from cassandra.cqlengine import columns
from cassandra.cqlengine import connection
from cassandra.cqlengine.models import Model
from qlient import Client, GraphQLResponse


class Attribute(Model):
    __table_name__ = "attributes"
    id = columns.UUID(primary_key=True, default=uuid4)
    constant_name = columns.Text()


class Category(Model):
    __table_name__ = "categories"
    id = columns.UUID(primary_key=True, default=uuid4)
    constant_name = columns.Text()


class SortOrder(Model):
    __table_name__ = "sort_orders"
    id = columns.UUID(primary_key=True, default=uuid4)
    constant_name = columns.Text()


class Source(Model):
    __table_name__ = "sources"
    id = columns.UUID(primary_key=True, default=uuid4)
    constant_name = columns.Text()


def write_constants_cassandra(file, name, db_constants):
    file.write("pub mod " + name + " {\n")
    file.write("    use uuid::Uuid;\n")
    constants = []
    for constant in db_constants:
        if None != constant.constant_name:
            constants.append(constant)
    constants.sort(key=lambda x: x.constant_name)
    for constant in constants:
        file.write(
            f"    pub const {constant.constant_name.upper()}: Uuid = Uuid::from_bytes([\n"
        )
        file.write(f"        //{constant.id}\n")
        file.write("        ")
        for i in range(0, 14):
            file.write("0x" + f"{constant.id.bytes[i]:x}".zfill(2) + ", ")
        file.write("0x" + f"{constant.id.bytes[14]:x}".zfill(2) + ",\n")
        file.write("        0x" + f"{constant.id.bytes[15]:x}".zfill(2) + ",\n")
        file.write(f"    ]);\n")
    file.write("}\n")


def write_constants(file, name, db_constants):
    file.write("pub mod " + name + " {\n")
    file.write("    use uuid::Uuid;\n")
    constants = []
    for constant in db_constants:
        if None != constant["constantName"]:
            constants.append(
                type(
                    "",
                    (object,),
                    {
                        "id": UUID(constant["id"]),
                        "constant_name": constant["constantName"],
                    },
                )()
            )
    constants.sort(key=lambda x: x.constant_name)
    for constant in constants:
        file.write(
            f"    pub const {constant.constant_name.upper()}: Uuid = Uuid::from_bytes([\n"
        )
        file.write(f"        //{constant.id}\n")
        file.write("        ")
        for i in range(0, 14):
            file.write("0x" + f"{constant.id.bytes[i]:x}".zfill(2) + ", ")
        file.write("0x" + f"{constant.id.bytes[14]:x}".zfill(2) + ",\n")
        file.write("        0x" + f"{constant.id.bytes[15]:x}".zfill(2) + ",\n")
        file.write(f"    ]);\n")
    file.write("}\n")


if __name__ == "__main__":
    connection.setup(["localhost"], "dealtech")

    # client = Client("http://bonfire.dev.dealtech.com/graphql")
    client = Client("http://localhost:5000/graphql")

    file = open("src/ids.rs", "w")

    file.write("#[allow(dead_code)]\n")
    attributes: GraphQLResponse = client.query.attributes(
        _fields=["id", "constantName"]
    ).data["attributes"]
    write_constants(file, "attribute_ids", attributes)
    file.write("\n")

    file.write("#[allow(dead_code)]\n")
    categories: GraphQLResponse = client.query.categories(
        _fields=["id", "constantName"]
    ).data["categories"]
    write_constants(file, "category_ids", categories)
    file.write("\n")

    file.write("#[allow(dead_code)]\n")
    categories: GraphQLResponse = client.query.sortOrders(
        _fields=["id", "constantName"]
    ).data["sortOrders"]
    write_constants(file, "sort_order_ids", categories)
    file.write("\n")

    file.write("#[allow(dead_code)]\n")
    write_constants_cassandra(file, "source_ids", Source.all())

    file.close()
