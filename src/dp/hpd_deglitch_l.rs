#[doc = "Register `HPD_DEGLITCH_L` reader"]
pub type R = crate::R<HpdDeglitchLSpec>;
#[doc = "Register `HPD_DEGLITCH_L` writer"]
pub type W = crate::W<HpdDeglitchLSpec>;
#[doc = "Field `HPD_DEGLITCH_L` reader - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal This register is HPD_DEGLITCH \\[7:0\\]. The default value is 0x5E for 280.75 us deglitch time."]
pub type HpdDeglitchLR = crate::FieldReader;
#[doc = "Field `HPD_DEGLITCH_L` writer - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal This register is HPD_DEGLITCH \\[7:0\\]. The default value is 0x5E for 280.75 us deglitch time."]
pub type HpdDeglitchLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal This register is HPD_DEGLITCH \\[7:0\\]. The default value is 0x5E for 280.75 us deglitch time."]
    #[inline(always)]
    pub fn hpd_deglitch_l(&self) -> HpdDeglitchLR {
        HpdDeglitchLR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HPD_DEGLITCH, which is counted at 24 MHz, is used to de-glitch the HPD signal This register is HPD_DEGLITCH \\[7:0\\]. The default value is 0x5E for 280.75 us deglitch time."]
    #[inline(always)]
    #[must_use]
    pub fn hpd_deglitch_l(&mut self) -> HpdDeglitchLW<HpdDeglitchLSpec> {
        HpdDeglitchLW::new(self, 0)
    }
}
#[doc = "DP HPD De-glitch Low Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hpd_deglitch_l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpd_deglitch_l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpdDeglitchLSpec;
impl crate::RegisterSpec for HpdDeglitchLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpd_deglitch_l::R`](R) reader structure"]
impl crate::Readable for HpdDeglitchLSpec {}
#[doc = "`write(|w| ..)` method takes [`hpd_deglitch_l::W`](W) writer structure"]
impl crate::Writable for HpdDeglitchLSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets HPD_DEGLITCH_L to value 0x5e"]
impl crate::Resettable for HpdDeglitchLSpec {
    const RESET_VALUE: u32 = 0x5e;
}
