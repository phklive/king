interface SummaryProps {
  text: string;
}

const Summary: React.FC<SummaryProps> = ({ text }) => {
  return (
    <div>
      <h1>{text}</h1>
    </div>
  );
};

export default Summary;
