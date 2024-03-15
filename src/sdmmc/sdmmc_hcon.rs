#[doc = "Register `SDMMC_HCON` reader"]
pub type R = crate::R<SdmmcHconSpec>;
#[doc = "Field `HCON` reader - Configuration Dependent. Hardware configurations selected by user before synthesizing core. Register values can be used to develop configuration- independent software drivers. \\[0\\]: CARD_TYPE 0: MMC_ONLY 1: SD_MMC \\[5:1\\]: NUM_CARDS - 1 \\[6\\]: H_BUS_TYPE 0: APB 1: AHB \\[9:7\\]: H_DATA_WIDTH 0: 16 bits 1: 32 bits 2: 64 bits others: reserved \\[15:10\\]: H_ADDR_WIDTH 0 to 7: reserved 8: 9 bits 9: 10 bits ... 31: 32 bits 32 to 63: reserved \\[17:16\\]: DMA_INTERFACE 0: none 1: DMA 1 2: DMA 2 3: DMA 3 \\[20:18\\]: GE_DMA_DATA_WIDTH 0: 16 bits 1: 32 bits 2: 64 bits others: reserved \\[21\\]: FIFO_RAM_INSIDE 0: outside 1: inside \\[22\\]: IMPLEMENT_HOLD_REG 0: no hold register 1: hold register \\[23\\]: SET_CLK_FALSE_PATH 0: no false path 1: false path set \\[25:24\\]: NUM_CLK_DIVIDER-1 \\[26\\]: AREA_OPTIMIZED 0: no area optimization 1: Area optimization"]
pub type HconR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Configuration Dependent. Hardware configurations selected by user before synthesizing core. Register values can be used to develop configuration- independent software drivers. \\[0\\]: CARD_TYPE 0: MMC_ONLY 1: SD_MMC \\[5:1\\]: NUM_CARDS - 1 \\[6\\]: H_BUS_TYPE 0: APB 1: AHB \\[9:7\\]: H_DATA_WIDTH 0: 16 bits 1: 32 bits 2: 64 bits others: reserved \\[15:10\\]: H_ADDR_WIDTH 0 to 7: reserved 8: 9 bits 9: 10 bits ... 31: 32 bits 32 to 63: reserved \\[17:16\\]: DMA_INTERFACE 0: none 1: DMA 1 2: DMA 2 3: DMA 3 \\[20:18\\]: GE_DMA_DATA_WIDTH 0: 16 bits 1: 32 bits 2: 64 bits others: reserved \\[21\\]: FIFO_RAM_INSIDE 0: outside 1: inside \\[22\\]: IMPLEMENT_HOLD_REG 0: no hold register 1: hold register \\[23\\]: SET_CLK_FALSE_PATH 0: no false path 1: false path set \\[25:24\\]: NUM_CLK_DIVIDER-1 \\[26\\]: AREA_OPTIMIZED 0: no area optimization 1: Area optimization"]
    #[inline(always)]
    pub fn hcon(&self) -> HconR {
        HconR::new(self.bits)
    }
}
#[doc = "Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_hcon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcHconSpec;
impl crate::RegisterSpec for SdmmcHconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_hcon::R`](R) reader structure"]
impl crate::Readable for SdmmcHconSpec {}
#[doc = "`reset()` method sets SDMMC_HCON to value 0"]
impl crate::Resettable for SdmmcHconSpec {
    const RESET_VALUE: u32 = 0;
}
