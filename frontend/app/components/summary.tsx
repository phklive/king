interface SummaryProps {
  text: string;
}

const Summary: React.FC<SummaryProps> = ({ text }) => {
  return (
    <div className="w-full h-96 bg-black p-4 border border-black rounded-lg">
      <h1 className="text-center text-2xl">Simulation summary:</h1>
      <p>{text}</p>
    </div>
  );
};

export default Summary;
