import Mark from 'mark.js'
import { Document, type EnrichedDocumentSearchResultSetUnit } from 'flexsearch'
const searchDataURL = '/api/posts/json'

declare global {
  // take from src/posts/json.rs
  interface Posts {
    id: number
    href: string
    title: string
    date: string
    body: string | null
  }
}

function init () {
  console.log('init')
  const searchBox = document.querySelector('#searchBox')
  if (searchBox === null) {
    return
  }
  const index = new Document<Posts, ['title', 'href', 'body']>({
    tokenize: 'reverse',
    document: {
      id: 'id',
      index: ['title', 'body'],
      store: ['title', 'href', 'body']
    }
  })

  fetch(searchDataURL)
    .then(async pages => await pages.json())
    .then((pages: Posts[]) => {
      for (let i = 0; i < pages.length; i++) {
        index.add(i, pages[i])
      }
    })

  searchBox.addEventListener('keyup', function (event) {
    const eventtarget = event.currentTarget as HTMLInputElement
    const searchResultsArea = document.getElementById('searchResults')!
    const query = eventtarget.value

    // Only trigger a search when 2 chars. at least have been provided
    if (query.length < 2) {
      searchResultsArea.style.display = 'none'
      return
    }

    // Display search results
    const results = index.search(query, 10, { enrich: true })
    renderResults(results)
    searchResultsArea.style.display = 'block'
  })
}

function renderResults (results: Array<EnrichedDocumentSearchResultSetUnit<Posts>>) {
  const searchResults = document.querySelector('#searchResults')!
  const querybox = document.getElementById('searchBox')! as HTMLInputElement
  const query = querybox.value
  const BODY_LENGTH = 100

  // Clear search result
  while (searchResults.firstChild != null) { searchResults.removeChild(searchResults.firstChild) }

  // Show message when results is empty
  if (results.length === 0) {
    const resultPage = document.createElement('div')
    resultPage.className = 'searchResultPage'
    resultPage.innerHTML = 'No results found for query "' + query + '"'
    searchResults.append(resultPage)
    return
  }

  const arr = results[0].result
  if (results.length > 1) {
    arr.concat(results[1].result)
  }

  arr.filter((element, index, self) =>
    self.findIndex(e => e.id === element.id) === index)

  const instance = new Mark(document.getElementById('searchResults')!)
  arr.forEach((result) => {
    const resultPage = document.createElement('div')
    resultPage.className = 'searchResultPage'

    const resultTitle = document.createElement('a')
    resultTitle.className = 'searchResultTitle'
    resultTitle.href = result.doc.href
    resultTitle.innerHTML = result.doc.title
    resultPage.append(resultTitle)

    const resultBody = document.createElement('div')
    resultBody.className = 'searchResultBody'
    const matchPos = result.doc.body?.indexOf(query) ?? -1
    const bodyStartPos = matchPos - BODY_LENGTH / 2 > 0 ? matchPos - BODY_LENGTH / 2 : 0
    resultBody.innerHTML = result.doc.body?.substr(bodyStartPos, BODY_LENGTH) ?? ''
    resultPage.append(resultBody)
    if (searchResults) {
      searchResults.append(resultPage)
    }
    instance.mark(query)
  })
}

window.onload = function () {
  init()
}
