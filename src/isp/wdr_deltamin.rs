#[doc = "Register `WDR_DELTAMIN` reader"]
pub type R = crate::R<WdrDeltaminSpec>;
#[doc = "Register `WDR_DELTAMIN` writer"]
pub type W = crate::W<WdrDeltaminSpec>;
#[doc = "Field `DMIN_THRESH` reader - Lower threshold for\n\ndeltaMin value unsigned 12 bit\n\nvalue\n\n"]
pub type DminThreshR = crate::FieldReader<u16>;
#[doc = "Field `DMIN_THRESH` writer - Lower threshold for\n\ndeltaMin value unsigned 12 bit\n\nvalue\n\n"]
pub type DminThreshW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMIN_STRENGTH` reader - strength factor for DMIN\n\nunsigned 5 bit value, range 0x00...0x10\n\n"]
pub type DminStrengthR = crate::FieldReader;
#[doc = "Field `DMIN_STRENGTH` writer - strength factor for DMIN\n\nunsigned 5 bit value, range 0x00...0x10\n\n"]
pub type DminStrengthW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:11 - Lower threshold for\n\ndeltaMin value unsigned 12 bit\n\nvalue\n\n"]
    #[inline(always)]
    pub fn dmin_thresh(&self) -> DminThreshR {
        DminThreshR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - strength factor for DMIN\n\nunsigned 5 bit value, range 0x00...0x10\n\n"]
    #[inline(always)]
    pub fn dmin_strength(&self) -> DminStrengthR {
        DminStrengthR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Lower threshold for\n\ndeltaMin value unsigned 12 bit\n\nvalue\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dmin_thresh(&mut self) -> DminThreshW<WdrDeltaminSpec> {
        DminThreshW::new(self, 0)
    }
    #[doc = "Bits 16:20 - strength factor for DMIN\n\nunsigned 5 bit value, range 0x00...0x10\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn dmin_strength(&mut self) -> DminStrengthW<WdrDeltaminSpec> {
        DminStrengthW::new(self, 16)
    }
}
#[doc = "DeltaMin Threshold and Strength factor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_deltamin::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_deltamin::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrDeltaminSpec;
impl crate::RegisterSpec for WdrDeltaminSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_deltamin::R`](R) reader structure"]
impl crate::Readable for WdrDeltaminSpec {}
#[doc = "`write(|w| ..)` method takes [`wdr_deltamin::W`](W) writer structure"]
impl crate::Writable for WdrDeltaminSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_DELTAMIN to value 0x0010_0000"]
impl crate::Resettable for WdrDeltaminSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
