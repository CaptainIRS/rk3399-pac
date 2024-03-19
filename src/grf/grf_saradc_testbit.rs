#[doc = "Register `GRF_SARADC_TESTBIT` reader"]
pub type R = crate::R<GrfSaradcTestbitSpec>;
#[doc = "Register `GRF_SARADC_TESTBIT` writer"]
pub type W = crate::W<GrfSaradcTestbitSpec>;
#[doc = "Field `SARADC_TESTBIT` reader - saradc test bit control"]
pub type SaradcTestbitR = crate::BitReader;
#[doc = "Field `SARADC_TESTBIT` writer - saradc test bit control"]
pub type SaradcTestbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - saradc test bit control"]
    #[inline(always)]
    pub fn saradc_testbit(&self) -> SaradcTestbitR {
        SaradcTestbitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - saradc test bit control"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_testbit(&mut self) -> SaradcTestbitW<GrfSaradcTestbitSpec> {
        SaradcTestbitW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSaradcTestbitSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "saradc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_saradc_testbit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_saradc_testbit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSaradcTestbitSpec;
impl crate::RegisterSpec for GrfSaradcTestbitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_saradc_testbit::R`](R) reader structure"]
impl crate::Readable for GrfSaradcTestbitSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_saradc_testbit::W`](W) writer structure"]
impl crate::Writable for GrfSaradcTestbitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SARADC_TESTBIT to value 0"]
impl crate::Resettable for GrfSaradcTestbitSpec {
    const RESET_VALUE: u32 = 0;
}
