#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcie_dma_channel_0_control: PcieDmaChannel0Control,
    pcie_dma_channel_0_start_pointer_lower: PcieDmaChannel0StartPointerLower,
    pcie_dma_channel_0_start_pointer_upper: PcieDmaChannel0StartPointerUpper,
    pcie_dma_channel_0_attribute_lower: PcieDmaChannel0AttributeLower,
    pcie_dma_channel_0_attribute_upper: PcieDmaChannel0AttributeUpper,
    pcie_dma_channel_1_control: PcieDmaChannel1Control,
    pcie_dma_channel_1_start_pointer_lower: PcieDmaChannel1StartPointerLower,
    pcie_dma_channel_1_start_pointer_upper: PcieDmaChannel1StartPointerUpper,
    pcie_dma_channel_1_attribute_lower: PcieDmaChannel1AttributeLower,
    pcie_dma_channel_1_attribute_upper: PcieDmaChannel1AttributeUpper,
    _reserved10: [u8; 0x78],
    pcie_dma_interrupt: PcieDmaInterrupt,
    pcie_dma_interrupt_enable: PcieDmaInterruptEnable,
    pcie_dma_interrupt_disable: PcieDmaInterruptDisable,
    pcie_dma_inbound_buffer_uncorrected_ecc_errors: PcieDmaInboundBufferUncorrectedEccErrors,
    pcie_dma_inbound_buffer_corrected_ecc_errors: PcieDmaInboundBufferCorrectedEccErrors,
    pcie_dma_outbound_buffer_uncorrected_ecc_errors: PcieDmaOutboundBufferUncorrectedEccErrors,
    pcie_dma_outbound_buffer_corrected_ecc_errors: PcieDmaOutboundBufferCorrectedEccErrors,
    _reserved17: [u8; 0x3c],
    pcie_dma_capability_and_version: PcieDmaCapabilityAndVersion,
    pcie_dma_configuration: PcieDmaConfiguration,
}
impl RegisterBlock {
    #[doc = "0x00 - PCIe DMA Channel 0 Control Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_control(&self) -> &PcieDmaChannel0Control {
        &self.pcie_dma_channel_0_control
    }
    #[doc = "0x04 - PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_start_pointer_lower(
        &self,
    ) -> &PcieDmaChannel0StartPointerLower {
        &self.pcie_dma_channel_0_start_pointer_lower
    }
    #[doc = "0x08 - PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_start_pointer_upper(
        &self,
    ) -> &PcieDmaChannel0StartPointerUpper {
        &self.pcie_dma_channel_0_start_pointer_upper
    }
    #[doc = "0x0c - PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_attribute_lower(&self) -> &PcieDmaChannel0AttributeLower {
        &self.pcie_dma_channel_0_attribute_lower
    }
    #[doc = "0x10 - PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_0_attribute_upper(&self) -> &PcieDmaChannel0AttributeUpper {
        &self.pcie_dma_channel_0_attribute_upper
    }
    #[doc = "0x14 - PCIe DMA Channel 1 Control Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_control(&self) -> &PcieDmaChannel1Control {
        &self.pcie_dma_channel_1_control
    }
    #[doc = "0x18 - PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_start_pointer_lower(
        &self,
    ) -> &PcieDmaChannel1StartPointerLower {
        &self.pcie_dma_channel_1_start_pointer_lower
    }
    #[doc = "0x1c - PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_start_pointer_upper(
        &self,
    ) -> &PcieDmaChannel1StartPointerUpper {
        &self.pcie_dma_channel_1_start_pointer_upper
    }
    #[doc = "0x20 - PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_attribute_lower(&self) -> &PcieDmaChannel1AttributeLower {
        &self.pcie_dma_channel_1_attribute_lower
    }
    #[doc = "0x24 - PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub const fn pcie_dma_channel_1_attribute_upper(&self) -> &PcieDmaChannel1AttributeUpper {
        &self.pcie_dma_channel_1_attribute_upper
    }
    #[doc = "0xa0 - PCIe DMA Interrupt Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt(&self) -> &PcieDmaInterrupt {
        &self.pcie_dma_interrupt
    }
    #[doc = "0xa4 - PCIe DMA Interrupt Enable Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt_enable(&self) -> &PcieDmaInterruptEnable {
        &self.pcie_dma_interrupt_enable
    }
    #[doc = "0xa8 - PCIe DMA Interrupt Disable Register"]
    #[inline(always)]
    pub const fn pcie_dma_interrupt_disable(&self) -> &PcieDmaInterruptDisable {
        &self.pcie_dma_interrupt_disable
    }
    #[doc = "0xac - PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_inbound_buffer_uncorrected_ecc_errors(
        &self,
    ) -> &PcieDmaInboundBufferUncorrectedEccErrors {
        &self.pcie_dma_inbound_buffer_uncorrected_ecc_errors
    }
    #[doc = "0xb0 - PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_inbound_buffer_corrected_ecc_errors(
        &self,
    ) -> &PcieDmaInboundBufferCorrectedEccErrors {
        &self.pcie_dma_inbound_buffer_corrected_ecc_errors
    }
    #[doc = "0xb4 - PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_outbound_buffer_uncorrected_ecc_errors(
        &self,
    ) -> &PcieDmaOutboundBufferUncorrectedEccErrors {
        &self.pcie_dma_outbound_buffer_uncorrected_ecc_errors
    }
    #[doc = "0xb8 - PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_outbound_buffer_corrected_ecc_errors(
        &self,
    ) -> &PcieDmaOutboundBufferCorrectedEccErrors {
        &self.pcie_dma_outbound_buffer_corrected_ecc_errors
    }
    #[doc = "0xf8 - PCIe DMA Capability and Version Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_capability_and_version(&self) -> &PcieDmaCapabilityAndVersion {
        &self.pcie_dma_capability_and_version
    }
    #[doc = "0xfc - PCIe DMA Configuration Register Reserved for future use"]
    #[inline(always)]
    pub const fn pcie_dma_configuration(&self) -> &PcieDmaConfiguration {
        &self.pcie_dma_configuration
    }
}
#[doc = "PCIE_DMA_CHANNEL_0_CONTROL (rw) register accessor: PCIe DMA Channel 0 Control Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_control`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_CONTROL")]
pub type PcieDmaChannel0Control =
    crate::Reg<pcie_dma_channel_0_control::PcieDmaChannel0ControlSpec>;
#[doc = "PCIe DMA Channel 0 Control Register Reserved for future use"]
pub mod pcie_dma_channel_0_control;
#[doc = "PCIE_DMA_CHANNEL_0_START_POINTER_LOWER (rw) register accessor: PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_start_pointer_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_START_POINTER_LOWER")]
pub type PcieDmaChannel0StartPointerLower =
    crate::Reg<pcie_dma_channel_0_start_pointer_lower::PcieDmaChannel0StartPointerLowerSpec>;
#[doc = "PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_0_start_pointer_lower;
#[doc = "PCIE_DMA_CHANNEL_0_START_POINTER_UPPER (rw) register accessor: PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_start_pointer_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_START_POINTER_UPPER")]
pub type PcieDmaChannel0StartPointerUpper =
    crate::Reg<pcie_dma_channel_0_start_pointer_upper::PcieDmaChannel0StartPointerUpperSpec>;
#[doc = "PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_0_start_pointer_upper;
#[doc = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_LOWER (rw) register accessor: PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_attribute_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_attribute_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_attribute_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_LOWER")]
pub type PcieDmaChannel0AttributeLower =
    crate::Reg<pcie_dma_channel_0_attribute_lower::PcieDmaChannel0AttributeLowerSpec>;
#[doc = "PCIe DMA Channel 0 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_0_attribute_lower;
#[doc = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER (rw) register accessor: PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_attribute_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_attribute_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_0_attribute_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER")]
pub type PcieDmaChannel0AttributeUpper =
    crate::Reg<pcie_dma_channel_0_attribute_upper::PcieDmaChannel0AttributeUpperSpec>;
#[doc = "PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_0_attribute_upper;
#[doc = "PCIE_DMA_CHANNEL_1_CONTROL (rw) register accessor: PCIe DMA Channel 1 Control Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_control`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_CONTROL")]
pub type PcieDmaChannel1Control =
    crate::Reg<pcie_dma_channel_1_control::PcieDmaChannel1ControlSpec>;
#[doc = "PCIe DMA Channel 1 Control Register Reserved for future use"]
pub mod pcie_dma_channel_1_control;
#[doc = "PCIE_DMA_CHANNEL_1_START_POINTER_LOWER (rw) register accessor: PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_start_pointer_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_start_pointer_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_start_pointer_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_START_POINTER_LOWER")]
pub type PcieDmaChannel1StartPointerLower =
    crate::Reg<pcie_dma_channel_1_start_pointer_lower::PcieDmaChannel1StartPointerLowerSpec>;
#[doc = "PCIe DMA Channel 1 Start Pointer Lower Register Lower 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_1_start_pointer_lower;
#[doc = "PCIE_DMA_CHANNEL_1_START_POINTER_UPPER (rw) register accessor: PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_start_pointer_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_start_pointer_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_start_pointer_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_START_POINTER_UPPER")]
pub type PcieDmaChannel1StartPointerUpper =
    crate::Reg<pcie_dma_channel_1_start_pointer_upper::PcieDmaChannel1StartPointerUpperSpec>;
#[doc = "PCIe DMA Channel 1 Start Pointer Upper Register Upper 32-bits Pointer Address Registers"]
pub mod pcie_dma_channel_1_start_pointer_upper;
#[doc = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER (rw) register accessor: PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_attribute_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_attribute_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_attribute_lower`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER")]
pub type PcieDmaChannel1AttributeLower =
    crate::Reg<pcie_dma_channel_1_attribute_lower::PcieDmaChannel1AttributeLowerSpec>;
#[doc = "PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_1_attribute_lower;
#[doc = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_UPPER (rw) register accessor: PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_attribute_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_attribute_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_channel_1_attribute_upper`]
module"]
#[doc(alias = "PCIE_DMA_CHANNEL_1_ATTRIBUTE_UPPER")]
pub type PcieDmaChannel1AttributeUpper =
    crate::Reg<pcie_dma_channel_1_attribute_upper::PcieDmaChannel1AttributeUpperSpec>;
#[doc = "PCIe DMA Channel 1 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub mod pcie_dma_channel_1_attribute_upper;
#[doc = "PCIE_DMA_INTERRUPT (rw) register accessor: PCIe DMA Interrupt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT")]
pub type PcieDmaInterrupt = crate::Reg<pcie_dma_interrupt::PcieDmaInterruptSpec>;
#[doc = "PCIe DMA Interrupt Register"]
pub mod pcie_dma_interrupt;
#[doc = "PCIE_DMA_INTERRUPT_ENABLE (rw) register accessor: PCIe DMA Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt_enable`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT_ENABLE")]
pub type PcieDmaInterruptEnable = crate::Reg<pcie_dma_interrupt_enable::PcieDmaInterruptEnableSpec>;
#[doc = "PCIe DMA Interrupt Enable Register"]
pub mod pcie_dma_interrupt_enable;
#[doc = "PCIE_DMA_INTERRUPT_DISABLE (rw) register accessor: PCIe DMA Interrupt Disable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_interrupt_disable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_interrupt_disable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_interrupt_disable`]
module"]
#[doc(alias = "PCIE_DMA_INTERRUPT_DISABLE")]
pub type PcieDmaInterruptDisable =
    crate::Reg<pcie_dma_interrupt_disable::PcieDmaInterruptDisableSpec>;
#[doc = "PCIe DMA Interrupt Disable Register"]
pub mod pcie_dma_interrupt_disable;
#[doc = "PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_uncorrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_inbound_buffer_uncorrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_INBOUND_BUFFER_UNCORRECTED_ECC_ERRORS")]
pub type PcieDmaInboundBufferUncorrectedEccErrors = crate::Reg<
    pcie_dma_inbound_buffer_uncorrected_ecc_errors::PcieDmaInboundBufferUncorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Inbound Buffer Uncorrected ECC Errors Reserved for future use"]
pub mod pcie_dma_inbound_buffer_uncorrected_ecc_errors;
#[doc = "PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_inbound_buffer_corrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_inbound_buffer_corrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_INBOUND_BUFFER_CORRECTED_ECC_ERRORS")]
pub type PcieDmaInboundBufferCorrectedEccErrors = crate::Reg<
    pcie_dma_inbound_buffer_corrected_ecc_errors::PcieDmaInboundBufferCorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Inbound Buffer corrected ECC Errors Reserved for future use"]
pub mod pcie_dma_inbound_buffer_corrected_ecc_errors;
#[doc = "PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_outbound_buffer_uncorrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_outbound_buffer_uncorrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_OUTBOUND_BUFFER_UNCORRECTED_ECC_ERRORS")]
pub type PcieDmaOutboundBufferUncorrectedEccErrors = crate::Reg<
    pcie_dma_outbound_buffer_uncorrected_ecc_errors::PcieDmaOutboundBufferUncorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Outbound Buffer Uncorrected ECC Errors Reserved for future use"]
pub mod pcie_dma_outbound_buffer_uncorrected_ecc_errors;
#[doc = "PCIE_DMA_OUTBOUND_BUFFER_CORRECTED_ECC_ERRORS (r) register accessor: PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_outbound_buffer_corrected_ecc_errors::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_outbound_buffer_corrected_ecc_errors`]
module"]
#[doc(alias = "PCIE_DMA_OUTBOUND_BUFFER_CORRECTED_ECC_ERRORS")]
pub type PcieDmaOutboundBufferCorrectedEccErrors = crate::Reg<
    pcie_dma_outbound_buffer_corrected_ecc_errors::PcieDmaOutboundBufferCorrectedEccErrorsSpec,
>;
#[doc = "PCIe DMA Outbound Buffer corrected ECC Errors Reserved for future use"]
pub mod pcie_dma_outbound_buffer_corrected_ecc_errors;
#[doc = "PCIE_DMA_CAPABILITY_AND_VERSION (r) register accessor: PCIe DMA Capability and Version Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_capability_and_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_capability_and_version`]
module"]
#[doc(alias = "PCIE_DMA_CAPABILITY_AND_VERSION")]
pub type PcieDmaCapabilityAndVersion =
    crate::Reg<pcie_dma_capability_and_version::PcieDmaCapabilityAndVersionSpec>;
#[doc = "PCIe DMA Capability and Version Register Reserved for future use"]
pub mod pcie_dma_capability_and_version;
#[doc = "PCIE_DMA_CONFIGURATION (r) register accessor: PCIe DMA Configuration Register Reserved for future use\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_configuration::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcie_dma_configuration`]
module"]
#[doc(alias = "PCIE_DMA_CONFIGURATION")]
pub type PcieDmaConfiguration = crate::Reg<pcie_dma_configuration::PcieDmaConfigurationSpec>;
#[doc = "PCIe DMA Configuration Register Reserved for future use"]
pub mod pcie_dma_configuration;
