#[doc = "Register `CONFIG3_ID` reader"]
pub type R = crate::R<Config3IdSpec>;
#[doc = "Field `CONFGPAUD` reader - Indicates that the audio interface is Generic Parallel Audio (GPAUD) Reset Value: (GPAUDPORTS== 1) ? 1 : 0"]
pub type ConfgpaudR = crate::BitReader;
#[doc = "Field `CONFAHBAUDDMA` reader - Indicates that the audio interface is AHB AUD DMA Reset Value: (AHBAUDDMAIF== 1) ? 1 : 0"]
pub type ConfahbauddmaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that the audio interface is Generic Parallel Audio (GPAUD) Reset Value: (GPAUDPORTS== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn confgpaud(&self) -> ConfgpaudR {
        ConfgpaudR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the audio interface is AHB AUD DMA Reset Value: (AHBAUDDMAIF== 1) ? 1 : 0"]
    #[inline(always)]
    pub fn confahbauddma(&self) -> ConfahbauddmaR {
        ConfahbauddmaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Indicates that the audio interface is Generic Parallel Audio (GPAUD)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config3_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config3IdSpec;
impl crate::RegisterSpec for Config3IdSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`config3_id::R`](R) reader structure"]
impl crate::Readable for Config3IdSpec {}
#[doc = "`reset()` method sets CONFIG3_ID to value 0"]
impl crate::Resettable for Config3IdSpec {
    const RESET_VALUE: u8 = 0;
}
