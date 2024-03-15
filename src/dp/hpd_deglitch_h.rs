#[doc = "Register `HPD_DEGLITCH_H` reader"]
pub type R = crate::R<HpdDeglitchHSpec>;
#[doc = "Register `HPD_DEGLITCH_H` writer"]
pub type W = crate::W<HpdDeglitchHSpec>;
#[doc = "Field `HPD_DEGLITCH_H` reader - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal. This register is HPD_DEGLITCH \\[13:8\\]. The default value is 0x1A for 280.75 us deglitch time."]
pub type HpdDeglitchHR = crate::FieldReader;
#[doc = "Field `HPD_DEGLITCH_H` writer - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal. This register is HPD_DEGLITCH \\[13:8\\]. The default value is 0x1A for 280.75 us deglitch time."]
pub type HpdDeglitchHW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal. This register is HPD_DEGLITCH \\[13:8\\]. The default value is 0x1A for 280.75 us deglitch time."]
    #[inline(always)]
    pub fn hpd_deglitch_h(&self) -> HpdDeglitchHR {
        HpdDeglitchHR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal. This register is HPD_DEGLITCH \\[13:8\\]. The default value is 0x1A for 280.75 us deglitch time."]
    #[inline(always)]
    #[must_use]
    pub fn hpd_deglitch_h(&mut self) -> HpdDeglitchHW<HpdDeglitchHSpec> {
        HpdDeglitchHW::new(self, 0)
    }
}
#[doc = "DP HPD De-glitch High Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpd_deglitch_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpd_deglitch_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpdDeglitchHSpec;
impl crate::RegisterSpec for HpdDeglitchHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpd_deglitch_h::R`](R) reader structure"]
impl crate::Readable for HpdDeglitchHSpec {}
#[doc = "`write(|w| ..)` method takes [`hpd_deglitch_h::W`](W) writer structure"]
impl crate::Writable for HpdDeglitchHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets HPD_DEGLITCH_H to value 0x1a"]
impl crate::Resettable for HpdDeglitchHSpec {
    const RESET_VALUE: u32 = 0x1a;
}
