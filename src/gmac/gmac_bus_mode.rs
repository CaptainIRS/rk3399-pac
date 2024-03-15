#[doc = "Register `GMAC_BUS_MODE` reader"]
pub type R = crate::R<GmacBusModeSpec>;
#[doc = "Register `GMAC_BUS_MODE` writer"]
pub type W = crate::W<GmacBusModeSpec>;
#[doc = "Field `SWR` reader - Software Reset When this bit is set, the MAC DMA Controller resets all GMAC Subsystem internal registers and logic. It is cleared automatically after the reset operation has completed in all of the core clock domains. Read a 0 value in this bit before re-programming any register of the core. Note: The reset operation is completed only when all the resets in all the active clock domains are de-asserted. Hence it is essential that all the PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion."]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset When this bit is set, the MAC DMA Controller resets all GMAC Subsystem internal registers and logic. It is cleared automatically after the reset operation has completed in all of the core clock domains. Read a 0 value in this bit before re-programming any register of the core. Note: The reset operation is completed only when all the resets in all the active clock domains are de-asserted. Hence it is essential that all the PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion."]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length This bit specifies the number of dword to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When DSL value equals zero, then the descriptor table is taken as contiguous by the DMA, in Ring mode."]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length This bit specifies the number of dword to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When DSL value equals zero, then the descriptor table is taken as contiguous by the DMA, in Ring mode."]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This will be the maximum value that is used in a single block Read/Write. The DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. When USP is set high, this PBL value is applicable for TxDMA transactions only. The PBL values have the following limitations. The maximum number of beats (PBL) possible is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA. The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified (as given below). For different data bus widths and FIFO sizes, the valid PBL range (including x8 mode) is provided in the following table. If the PBL is common for both transmit and receive DMA, the minimum Rx FIFO and Tx FIFO depths must be considered. Do not program out-of-range PBL values, because the system may not behave properly. For TxFIFO, valid PBL range in full duplex mode and duplex mode is 128 or less. For RxFIFO, valid PBL range in full duplex mode is all."]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This will be the maximum value that is used in a single block Read/Write. The DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. When USP is set high, this PBL value is applicable for TxDMA transactions only. The PBL values have the following limitations. The maximum number of beats (PBL) possible is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA. The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified (as given below). For different data bus widths and FIFO sizes, the valid PBL range (including x8 mode) is provided in the following table. If the PBL is common for both transmit and receive DMA, the minimum Rx FIFO and Tx FIFO depths must be considered. Do not program out-of-range PBL values, because the system may not behave properly. For TxFIFO, valid PBL range in full duplex mode and duplex mode is 128 or less. For RxFIFO, valid PBL range in full duplex mode is all."]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FB` reader - Fixed Burst This bit controls whether the AXI Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AXI will use SINGLE and INCR burst transfer operations."]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst This bit controls whether the AXI Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AXI will use SINGLE and INCR burst transfer operations."]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - RxDMA PBL These bits indicate the maximum number of beats to be transferred in one RxDMA transaction. This will be the maximum value that is used in a single block Read/Write. The RxDMA will always attempt to burst as specified in RPBL each time it starts a Burst transfer on the host bus. RPBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. These bits are valid and applicable only when USP is set high."]
pub type RpblR = crate::FieldReader;
#[doc = "Field `RPBL` writer - RxDMA PBL These bits indicate the maximum number of beats to be transferred in one RxDMA transaction. This will be the maximum value that is used in a single block Read/Write. The RxDMA will always attempt to burst as specified in RPBL each time it starts a Burst transfer on the host bus. RPBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. These bits are valid and applicable only when USP is set high."]
pub type RpblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Separate PBL When set high, it configures the RxDMA to use the value configured in bits \\[22:17\\]
as PBL while the PBL value in bits \\[13:8\\]
is applicable to TxDMA operations only. When reset to low, the PBL value in bits \\[13:8\\]
is applicable for both DMA engines."]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - Use Separate PBL When set high, it configures the RxDMA to use the value configured in bits \\[22:17\\]
as PBL while the PBL value in bits \\[13:8\\]
is applicable to TxDMA operations only. When reset to low, the PBL value in bits \\[13:8\\]
is applicable for both DMA engines."]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL_MODE` reader - 8xPBL Mode When set high, this bit multiplies the PBL value programmed (bits \\[22:17\\]
and bits \\[13:8\\]) eight times. Thus the DMA will transfer data in to a maximum of 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
pub type PblModeR = crate::BitReader;
#[doc = "Field `PBL_MODE` writer - 8xPBL Mode When set high, this bit multiplies the PBL value programmed (bits \\[22:17\\]
and bits \\[13:8\\]) eight times. Thus the DMA will transfer data in to a maximum of 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
pub type PblModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats When this bit is set high and the FB bit equals 1, the AXI interface generates all bursts aligned to the start address LS bits. If the FB bit equals 0, the first burst (accessing the data buffer's start address) is not aligned, but subsequent bursts are aligned to the address."]
pub type AalR = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats When this bit is set high and the FB bit equals 1, the AXI interface generates all bursts aligned to the start address LS bits. If the FB bit equals 0, the first burst (accessing the data buffer's start address) is not aligned, but subsequent bursts are aligned to the address."]
pub type AalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets all GMAC Subsystem internal registers and logic. It is cleared automatically after the reset operation has completed in all of the core clock domains. Read a 0 value in this bit before re-programming any register of the core. Note: The reset operation is completed only when all the resets in all the active clock domains are de-asserted. Hence it is essential that all the PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion."]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of dword to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When DSL value equals zero, then the descriptor table is taken as contiguous by the DMA, in Ring mode."]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This will be the maximum value that is used in a single block Read/Write. The DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. When USP is set high, this PBL value is applicable for TxDMA transactions only. The PBL values have the following limitations. The maximum number of beats (PBL) possible is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA. The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified (as given below). For different data bus widths and FIFO sizes, the valid PBL range (including x8 mode) is provided in the following table. If the PBL is common for both transmit and receive DMA, the minimum Rx FIFO and Tx FIFO depths must be considered. Do not program out-of-range PBL values, because the system may not behave properly. For TxFIFO, valid PBL range in full duplex mode and duplex mode is 128 or less. For RxFIFO, valid PBL range in full duplex mode is all."]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AXI Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AXI will use SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - RxDMA PBL These bits indicate the maximum number of beats to be transferred in one RxDMA transaction. This will be the maximum value that is used in a single block Read/Write. The RxDMA will always attempt to burst as specified in RPBL each time it starts a Burst transfer on the host bus. RPBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. These bits are valid and applicable only when USP is set high."]
    #[inline(always)]
    pub fn rpbl(&self) -> RpblR {
        RpblR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, it configures the RxDMA to use the value configured in bits \\[22:17\\]
as PBL while the PBL value in bits \\[13:8\\]
is applicable to TxDMA operations only. When reset to low, the PBL value in bits \\[13:8\\]
is applicable for both DMA engines."]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 8xPBL Mode When set high, this bit multiplies the PBL value programmed (bits \\[22:17\\]
and bits \\[13:8\\]) eight times. Thus the DMA will transfer data in to a maximum of 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
    #[inline(always)]
    pub fn pbl_mode(&self) -> PblModeR {
        PblModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-Aligned Beats When this bit is set high and the FB bit equals 1, the AXI interface generates all bursts aligned to the start address LS bits. If the FB bit equals 0, the first burst (accessing the data buffer's start address) is not aligned, but subsequent bursts are aligned to the address."]
    #[inline(always)]
    pub fn aal(&self) -> AalR {
        AalR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset When this bit is set, the MAC DMA Controller resets all GMAC Subsystem internal registers and logic. It is cleared automatically after the reset operation has completed in all of the core clock domains. Read a 0 value in this bit before re-programming any register of the core. Note: The reset operation is completed only when all the resets in all the active clock domains are de-asserted. Hence it is essential that all the PHY inputs clocks (applicable for the selected PHY interface) are present for software reset completion."]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SwrW<GmacBusModeSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length This bit specifies the number of dword to skip between two unchained descriptors. The address skipping starts from the end of current descriptor to the start of next descriptor. When DSL value equals zero, then the descriptor table is taken as contiguous by the DMA, in Ring mode."]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DslW<GmacBusModeSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length These bits indicate the maximum number of beats to be transferred in one DMA transaction. This will be the maximum value that is used in a single block Read/Write. The DMA will always attempt to burst as specified in PBL each time it starts a Burst transfer on the host bus. PBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. When USP is set high, this PBL value is applicable for TxDMA transactions only. The PBL values have the following limitations. The maximum number of beats (PBL) possible is limited by the size of the Tx FIFO and Rx FIFO in the MTL layer and the data bus width on the DMA. The FIFO has a constraint that the maximum beat supported is half the depth of the FIFO, except when specified (as given below). For different data bus widths and FIFO sizes, the valid PBL range (including x8 mode) is provided in the following table. If the PBL is common for both transmit and receive DMA, the minimum Rx FIFO and Tx FIFO depths must be considered. Do not program out-of-range PBL values, because the system may not behave properly. For TxFIFO, valid PBL range in full duplex mode and duplex mode is 128 or less. For RxFIFO, valid PBL range in full duplex mode is all."]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PblW<GmacBusModeSpec> {
        PblW::new(self, 8)
    }
    #[doc = "Bit 16 - Fixed Burst This bit controls whether the AXI Master interface performs fixed burst transfers or not. When set, the AHB will use only SINGLE, INCR4, INCR8 or INCR16 during start of normal burst transfers. When reset, the AXI will use SINGLE and INCR burst transfer operations."]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FbW<GmacBusModeSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bits 17:22 - RxDMA PBL These bits indicate the maximum number of beats to be transferred in one RxDMA transaction. This will be the maximum value that is used in a single block Read/Write. The RxDMA will always attempt to burst as specified in RPBL each time it starts a Burst transfer on the host bus. RPBL can be programmed with permissible values of 1, 2, 4, 8, 16, and 32. Any other value will result in undefined behavior. These bits are valid and applicable only when USP is set high."]
    #[inline(always)]
    #[must_use]
    pub fn rpbl(&mut self) -> RpblW<GmacBusModeSpec> {
        RpblW::new(self, 17)
    }
    #[doc = "Bit 23 - Use Separate PBL When set high, it configures the RxDMA to use the value configured in bits \\[22:17\\]
as PBL while the PBL value in bits \\[13:8\\]
is applicable to TxDMA operations only. When reset to low, the PBL value in bits \\[13:8\\]
is applicable for both DMA engines."]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> UspW<GmacBusModeSpec> {
        UspW::new(self, 23)
    }
    #[doc = "Bit 24 - 8xPBL Mode When set high, this bit multiplies the PBL value programmed (bits \\[22:17\\]
and bits \\[13:8\\]) eight times. Thus the DMA will transfer data in to a maximum of 8, 16, 32, 64, 128, and 256 beats depending on the PBL value."]
    #[inline(always)]
    #[must_use]
    pub fn pbl_mode(&mut self) -> PblModeW<GmacBusModeSpec> {
        PblModeW::new(self, 24)
    }
    #[doc = "Bit 25 - Address-Aligned Beats When this bit is set high and the FB bit equals 1, the AXI interface generates all bursts aligned to the start address LS bits. If the FB bit equals 0, the first burst (accessing the data buffer's start address) is not aligned, but subsequent bursts are aligned to the address."]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AalW<GmacBusModeSpec> {
        AalW::new(self, 25)
    }
}
#[doc = "Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_bus_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_bus_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacBusModeSpec;
impl crate::RegisterSpec for GmacBusModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_bus_mode::R`](R) reader structure"]
impl crate::Readable for GmacBusModeSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_bus_mode::W`](W) writer structure"]
impl crate::Writable for GmacBusModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_BUS_MODE to value 0x0002_0101"]
impl crate::Resettable for GmacBusModeSpec {
    const RESET_VALUE: u32 = 0x0002_0101;
}
