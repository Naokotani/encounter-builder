<script>
  let isOpen = false;

  function openModal() {
    isOpen = true;
  }

  function closeModal() {
    isOpen = false;
  }
</script>

{#if isOpen}
  <div class="modal">
    <div class="modal-content">
      <h2>Encounter Builder Instructions</h2>
      <p>
				Pick the budget you want to assign to each monster type.
				Setting a monster budget to 'all' will disable the monsters
				below it. 'None' will remove that monster group from the
				results. 'More', 'Half' and 'less' represents the amount of
				remaining budget that will be used on that monster group.
			</p>
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
  }

  .modal-content {
    background: var(--modalColor);
		max-width: 60%;
    padding: 20px;
    border-radius: 5px;
    box-shadow: var(--level-4);
  }

	button {
		margin-bottom: 1rem;
	}
</style>
