import { NavLink, Outlet, useSearchParams } from "react-router-dom";
import { getAllInvoices, Invoice } from "../invoicesRepository";

function SearchInput() {
  const [searchParams, setSearchParams] = useSearchParams();

  const onChange = (e: React.FormEvent<HTMLInputElement>) => {
    const filter = e.currentTarget.value;
    filter ? setSearchParams({ filter }) : setSearchParams({});
  };
  return <input value={searchParams.get("filter") || ""} onChange={onChange} />;
}

export default function Invoices() {
  const invoices: Invoice[] = getAllInvoices();
  const [searchParams, _] = useSearchParams();

  return (
    <div style={{ display: "flex" }}>
      <nav style={{ borderRight: "solid 1px", padding: "1rem" }}>
        <SearchInput />
        {invoices
          .filter((invoice) => {
            const filter = searchParams.get("filter");
            if (!filter) {
              return true;
            }

            const name = invoice.name.toLowerCase();
            return name.startsWith(filter.toLowerCase());
          })
          .map((invoice) => (
            <NavLink
              style={({ isActive }) => {
                return {
                  display: "block",
                  margin: "1rem 0",
                  color: isActive ? "red" : "blue",
                };
              }}
              to={`/invoices/${invoice.number}`}
              key={invoice.number}
            >
              {invoice.name}
            </NavLink>
          ))}
      </nav>
      <Outlet />
    </div>
  );
}
