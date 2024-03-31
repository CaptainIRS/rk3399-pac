#[doc = "Register `Bandwidth` reader"]
pub type R = crate::R<BandwidthSpec>;
#[doc = "Register `Bandwidth` writer"]
pub type W = crate::W<BandwidthSpec>;
#[doc = "Field `BANDWIDTH` reader - Field0000 Abstract\n\nIn Bandwidth Limiter or Bandwidth Regulator mode, the bandwidth\n\nthreshold in units of 1/256th bytes per cycle. For example, 80 MBps\n\non a 250 MHz interface is value 0x0052. The valid bits may be\n\ndifferent for different master NIU."]
pub type BandwidthR = crate::FieldReader<u16>;
#[doc = "Field `BANDWIDTH` writer - Field0000 Abstract\n\nIn Bandwidth Limiter or Bandwidth Regulator mode, the bandwidth\n\nthreshold in units of 1/256th bytes per cycle. For example, 80 MBps\n\non a 250 MHz interface is value 0x0052. The valid bits may be\n\ndifferent for different master NIU."]
pub type BandwidthW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - Field0000 Abstract\n\nIn Bandwidth Limiter or Bandwidth Regulator mode, the bandwidth\n\nthreshold in units of 1/256th bytes per cycle. For example, 80 MBps\n\non a 250 MHz interface is value 0x0052. The valid bits may be\n\ndifferent for different master NIU."]
    #[inline(always)]
    pub fn bandwidth(&self) -> BandwidthR {
        BandwidthR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Field0000 Abstract\n\nIn Bandwidth Limiter or Bandwidth Regulator mode, the bandwidth\n\nthreshold in units of 1/256th bytes per cycle. For example, 80 MBps\n\non a 250 MHz interface is value 0x0052. The valid bits may be\n\ndifferent for different master NIU."]
    #[inline(always)]
    #[must_use]
    pub fn bandwidth(&mut self) -> BandwidthW<BandwidthSpec> {
        BandwidthW::new(self, 0)
    }
}
#[doc = "Bandwidth register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bandwidth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bandwidth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BandwidthSpec;
impl crate::RegisterSpec for BandwidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bandwidth::R`](R) reader structure"]
impl crate::Readable for BandwidthSpec {}
#[doc = "`write(|w| ..)` method takes [`bandwidth::W`](W) writer structure"]
impl crate::Writable for BandwidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Bandwidth to value 0x018a"]
impl crate::Resettable for BandwidthSpec {
    const RESET_VALUE: u32 = 0x018a;
}
