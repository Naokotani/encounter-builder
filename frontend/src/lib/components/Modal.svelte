<script>
	import { goto } from '$app/navigation';
	import { Instructions } from '$lib'
  let isOpen = false;

  function openModal() {
		if (window.matchMedia('(max-width: 767px)').matches) {
			goto('/encounter-builder/instructions')
		} else {
			isOpen = true;
		}
  }
 
  function closeModal() {
    isOpen = false;
  }

</script>

{#if isOpen}
  <div class="modal">
    <div class="modal-content">
			<Instructions />
      <button type="button" on:click={closeModal}>Close</button>
    </div>
  </div>
{/if}

<button type="button" on:click={openModal}>Instructions</button>

<style>
  .modal {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
		color: var(--modalText);
			padding: 20px;
			width: 100%;
  }

  .modal-content {
    background: var(--modalColor);
    border-radius: 5px;
    box-shadow: var(--level-4);
		padding: 20px;
		max-width: 100%;
		max-width: 60%
  }

	button {
		margin-bottom: 1rem;
	}
</style>
