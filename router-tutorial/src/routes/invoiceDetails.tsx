import { useNavigate, useParams } from "react-router-dom";
import { deleteInvoice, getInvoice, Invoice } from "../invoicesRepository";

type InvoiceParams = {
  invoiceId: string;
};


function DeleteInvoiceButton(props: {invoiceNumber: number}) {
  const navigate = useNavigate();
  return (
    <button
      onClick={() => {
        deleteInvoice(props.invoiceNumber);
        navigate("/invoices");
      }}
    >
      Delete
    </button>
  );
}

export default function InvoiceDetails(): JSX.Element {
  const params = useParams() as InvoiceParams;
  const invoiceId = parseInt(params.invoiceId, 10);
  const invoice = getInvoice(invoiceId) as Invoice;

  return (
    <main>
      <h2>Total Due: {invoice.amount}</h2>
      <p>
        {invoice.name}: {invoice.number}
      </p>
      <p>Due date: {invoice.due}</p>
      <p>
        <DeleteInvoiceButton invoiceNumber={invoice.number} />
      </p>
    </main>
  );
}
