#[doc = "Register `UART_CPR` reader"]
pub type R = crate::R<UartCprSpec>;
#[doc = "Field `APB_DATA_WIDTH` reader - 00 = 8 bits\n\n01 = 16 bits\n\n10 = 32 bits\n\n11 = reserved"]
pub type ApbDataWidthR = crate::FieldReader;
#[doc = "Field `AFCE_MODE` reader - 0 = FALSE\n\n1 = TRUE"]
pub type AfceModeR = crate::BitReader;
#[doc = "Field `THRE_MODE` reader - 0 = FALSE\n\n1 = TRUE"]
pub type ThreModeR = crate::BitReader;
#[doc = "Field `SIR_MODE` reader - 0 = FALSE\n\n1 = TRUE"]
pub type SirModeR = crate::BitReader;
#[doc = "Field `SIR_LP_MODE` reader - 0 = FALSE\n\n1 = TRUE"]
pub type SirLpModeR = crate::BitReader;
#[doc = "Field `NEW_FEAT` reader - 0 = FALSE\n\n1 = TRUE"]
pub type NewFeatR = crate::BitReader;
#[doc = "Field `FIFO_ACCESS` reader - 0 = FALSE\n\n1 = TRUE"]
pub type FifoAccessR = crate::BitReader;
#[doc = "Field `FIFO_STAT` reader - 0 = FALSE\n\n1 = TRUE"]
pub type FifoStatR = crate::BitReader;
#[doc = "Field `SHADOW` reader - 0 = FALSE\n\n1 = TRUE"]
pub type ShadowR = crate::BitReader;
#[doc = "Field `UART_ADD_ENCODED_PARAMS` reader - 0 = FALSE\n\n1 = TRUE"]
pub type UartAddEncodedParamsR = crate::BitReader;
#[doc = "Field `DMA_EXTRA` reader - 0 = FALSE\n\n1 = TRUE"]
pub type DmaExtraR = crate::BitReader;
#[doc = "Field `FIFO_MODE` reader - 0x00 = 0\n\n0x01 = 16\n\n0x02 = 32\n\nto\n\n0x80 = 2048\n\n0x81- 0xff = reserved"]
pub type FifoModeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 00 = 8 bits\n\n01 = 16 bits\n\n10 = 32 bits\n\n11 = reserved"]
    #[inline(always)]
    pub fn apb_data_width(&self) -> ApbDataWidthR {
        ApbDataWidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn afce_mode(&self) -> AfceModeR {
        AfceModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn thre_mode(&self) -> ThreModeR {
        ThreModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn sir_mode(&self) -> SirModeR {
        SirModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn sir_lp_mode(&self) -> SirLpModeR {
        SirLpModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn new_feat(&self) -> NewFeatR {
        NewFeatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn fifo_access(&self) -> FifoAccessR {
        FifoAccessR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn fifo_stat(&self) -> FifoStatR {
        FifoStatR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn shadow(&self) -> ShadowR {
        ShadowR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn uart_add_encoded_params(&self) -> UartAddEncodedParamsR {
        UartAddEncodedParamsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0 = FALSE\n\n1 = TRUE"]
    #[inline(always)]
    pub fn dma_extra(&self) -> DmaExtraR {
        DmaExtraR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 0x00 = 0\n\n0x01 = 16\n\n0x02 = 32\n\nto\n\n0x80 = 2048\n\n0x81- 0xff = reserved"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FifoModeR {
        FifoModeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Component Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_cpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartCprSpec;
impl crate::RegisterSpec for UartCprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_cpr::R`](R) reader structure"]
impl crate::Readable for UartCprSpec {}
#[doc = "`reset()` method sets UART_CPR to value 0"]
impl crate::Resettable for UartCprSpec {
    const RESET_VALUE: u32 = 0;
}
