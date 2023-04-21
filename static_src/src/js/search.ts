const searchDataURL = '/api/posts/json'
import Mark from 'mark.js'
import { Document, EnrichedDocumentSearchResultSetUnitResultUnit } from 'flexsearch'

declare global {
	//take from src/posts/json.rs
	interface Posts {
		id: number,
		href: string,
		title: string,
		date: string,
		body: string | null,
	}
}

function init() {
	const searchBox = document.querySelector('#searchBox')
	if (searchBox === null) {
		return
	}
	let index = new Document({
		tokenize: 'reverse',
		document: {
			id: 'doc',
			index: ['title', 'body'],
			store: ['title', 'href', 'body']
		}
	});

	fetch(searchDataURL)
		.then(pages => pages.json())
		.then((pages: Posts[]) => {
			for (let i = 0; i < pages.length; i++) {
				index.add(i, pages[i]);
			}
		})

	searchBox.addEventListener('keyup', function (event) {
		let eventtarget = event.currentTarget as HTMLInputElement;
		let searchResultsArea = document.getElementById('searchResults')!;
		let query = eventtarget.value;

		// Only trigger a search when 2 chars. at least have been provided
		if (query.length < 2) {
			searchResultsArea.style.display = 'none'
			return
		}

		// Display search results
		let results = index.search(query, 10, { enrich: true });
		renderResults(results);
		searchResultsArea.style.display = 'block'
	})
}

/**
 * Rendering search results
 * @param {Object[]} results Array of search results ( fields[] => { field, result[] => { document }} )
 */
function renderResults(results: EnrichedDocumentSearchResultSetUnitResultUnit<Posts>) {
	const searchResults = document.querySelector('#searchResults')!;
	const querybox = document.getElementById('searchBox')! as HTMLInputElement;
	const query = querybox.value;
	const BODY_LENGTH = 100

	// Clear search result
	while (searchResults.firstChild)
		searchResults.removeChild(searchResults.firstChild)

	// Show message when results is empty
	if (!results.length) {
		let resultPage = document.createElement('div')
		resultPage.className = 'searchResultPage'
		resultPage.innerHTML = 'No results found for query "' + query + '"'
		searchResults.append(resultPage)
		return
	}

	let arr = results[0].result;
	if (results.length > 1) {
		arr.concat(results[1].result)
	}

	arr.filter((element: any, index: number, self: any) =>
		self.findIndex(e => e.id === element.id) === index)

	let instance = new Mark(document.getElementById('searchResults')!)
	arr.forEach((result: any[]) => {
		let resultPage = document.createElement('div')
		resultPage.className = 'searchResultPage'

		let resultTitle = document.createElement('a')
		resultTitle.className = 'searchResultTitle'
		resultTitle.href = result.doc.href
		resultTitle.innerHTML = result.doc.title
		resultPage.append(resultTitle)

		let resultBody = document.createElement('div')
		resultBody.className = 'searchResultBody'
		let matchPos = result.doc.body.indexOf(query)
		let bodyStartPos = matchPos - BODY_LENGTH / 2 > 0 ? matchPos - BODY_LENGTH / 2 : 0
		resultBody.innerHTML = result.doc.body.substr(bodyStartPos, BODY_LENGTH)
		resultPage.append(resultBody)
		if (searchResults) {
			searchResults.append(resultPage)
		}
		instance.mark(query)
	})
}

init();
