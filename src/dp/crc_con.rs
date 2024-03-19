#[doc = "Register `CRC_CON` reader"]
pub type R = crate::R<CrcConSpec>;
#[doc = "Register `CRC_CON` writer"]
pub type W = crate::W<CrcConSpec>;
#[doc = "Field `PSR_VID_CRC_ENABLE` reader - PSR Video CRC enable. 0: Disable, 1: Enable"]
pub type PsrVidCrcEnableR = crate::BitReader;
#[doc = "Field `PSR_VID_CRC_ENABLE` writer - PSR Video CRC enable. 0: Disable, 1: Enable"]
pub type PsrVidCrcEnableW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PSR_VID_CRC_FLUSH` reader - PSR Video CRC flush enable. The PSR video CRC \n\nvalue is initialized at every v-sync leading edge."]
pub type PsrVidCrcFlushR = crate::BitReader;
#[doc = "Field `PSR_VID_CRC_FLUSH` writer - PSR Video CRC flush enable. The PSR video CRC \n\nvalue is initialized at every v-sync leading edge."]
pub type PsrVidCrcFlushW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - PSR Video CRC enable. 0: Disable, 1: Enable"]
    #[inline(always)]
    pub fn psr_vid_crc_enable(&self) -> PsrVidCrcEnableR {
        PsrVidCrcEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PSR Video CRC flush enable. The PSR video CRC \n\nvalue is initialized at every v-sync leading edge."]
    #[inline(always)]
    pub fn psr_vid_crc_flush(&self) -> PsrVidCrcFlushR {
        PsrVidCrcFlushR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PSR Video CRC enable. 0: Disable, 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn psr_vid_crc_enable(&mut self) -> PsrVidCrcEnableW<CrcConSpec> {
        PsrVidCrcEnableW::new(self, 0)
    }
    #[doc = "Bit 2 - PSR Video CRC flush enable. The PSR video CRC \n\nvalue is initialized at every v-sync leading edge."]
    #[inline(always)]
    #[must_use]
    pub fn psr_vid_crc_flush(&mut self) -> PsrVidCrcFlushW<CrcConSpec> {
        PsrVidCrcFlushW::new(self, 2)
    }
}
#[doc = "CRC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_con::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_con::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcConSpec;
impl crate::RegisterSpec for CrcConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_con::R`](R) reader structure"]
impl crate::Readable for CrcConSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_con::W`](W) writer structure"]
impl crate::Writable for CrcConSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x05;
}
#[doc = "`reset()` method sets CRC_CON to value 0"]
impl crate::Resettable for CrcConSpec {
    const RESET_VALUE: u32 = 0;
}
