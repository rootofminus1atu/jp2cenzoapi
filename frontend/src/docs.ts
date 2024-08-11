import './app.css';
import Docs from './Docs.svelte';

const app = new Docs({
	target: document.getElementById('app')!,
});

export default Docs;
