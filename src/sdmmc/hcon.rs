#[doc = "Register `HCON` reader"]
pub type R = crate::R<HconSpec>;
#[doc = "Field `HCON` reader - Configuration Dependent.\n\nHardware configurations selected by user before synthesizing\n\ncore. Register values can be used to develop configuration-\n\nindependent software drivers.\n\n\\[0\\]: CARD_TYPE\n\n0: MMC_ONLY\n\n1: SD_MMC\n\n\\[5:1\\]: NUM_CARDS - 1\n\n\\[6\\]: H_BUS_TYPE\n\n0: APB\n\n1: AHB\n\n\\[9:7\\]: H_DATA_WIDTH\n\n0: 16 bits\n\n1: 32 bits\n\n2: 64 bits\n\nothers: reserved\n\n\\[15:10\\]: H_ADDR_WIDTH\n\n0 to 7: reserved\n\n8: 9 bits\n\n9: 10 bits\n\n...\n\n31: 32 bits\n\n32 to 63: reserved\n\n\\[17:16\\]: DMA_INTERFACE\n\n0: none\n\n1: DMA 1\n\n2: DMA 2\n\n3: DMA 3\n\n\\[20:18\\]: GE_DMA_DATA_WIDTH\n\n0: 16 bits\n\n1: 32 bits\n\n2: 64 bits\n\nothers: reserved\n\n\\[21\\]: FIFO_RAM_INSIDE\n\n0: outside\n\n1: inside\n\n\\[22\\]: IMPLEMENT_HOLD_REG\n\n0: no hold register\n\n1: hold register\n\n\\[23\\]: SET_CLK_FALSE_PATH\n\n0: no false path\n\n1: false path set\n\n\\[25:24\\]: NUM_CLK_DIVIDER-1\n\n\\[26\\]: AREA_OPTIMIZED\n\n0: no area optimization\n\n1: Area optimization"]
pub type HconR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Configuration Dependent.\n\nHardware configurations selected by user before synthesizing\n\ncore. Register values can be used to develop configuration-\n\nindependent software drivers.\n\n\\[0\\]: CARD_TYPE\n\n0: MMC_ONLY\n\n1: SD_MMC\n\n\\[5:1\\]: NUM_CARDS - 1\n\n\\[6\\]: H_BUS_TYPE\n\n0: APB\n\n1: AHB\n\n\\[9:7\\]: H_DATA_WIDTH\n\n0: 16 bits\n\n1: 32 bits\n\n2: 64 bits\n\nothers: reserved\n\n\\[15:10\\]: H_ADDR_WIDTH\n\n0 to 7: reserved\n\n8: 9 bits\n\n9: 10 bits\n\n...\n\n31: 32 bits\n\n32 to 63: reserved\n\n\\[17:16\\]: DMA_INTERFACE\n\n0: none\n\n1: DMA 1\n\n2: DMA 2\n\n3: DMA 3\n\n\\[20:18\\]: GE_DMA_DATA_WIDTH\n\n0: 16 bits\n\n1: 32 bits\n\n2: 64 bits\n\nothers: reserved\n\n\\[21\\]: FIFO_RAM_INSIDE\n\n0: outside\n\n1: inside\n\n\\[22\\]: IMPLEMENT_HOLD_REG\n\n0: no hold register\n\n1: hold register\n\n\\[23\\]: SET_CLK_FALSE_PATH\n\n0: no false path\n\n1: false path set\n\n\\[25:24\\]: NUM_CLK_DIVIDER-1\n\n\\[26\\]: AREA_OPTIMIZED\n\n0: no area optimization\n\n1: Area optimization"]
    #[inline(always)]
    pub fn hcon(&self) -> HconR {
        HconR::new(self.bits)
    }
}
#[doc = "Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HconSpec;
impl crate::RegisterSpec for HconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcon::R`](R) reader structure"]
impl crate::Readable for HconSpec {}
#[doc = "`reset()` method sets HCON to value 0"]
impl crate::Resettable for HconSpec {
    const RESET_VALUE: u32 = 0;
}
