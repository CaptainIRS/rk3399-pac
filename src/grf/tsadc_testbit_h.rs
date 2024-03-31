#[doc = "Register `TSADC_TESTBIT_H` reader"]
pub type R = crate::R<TsadcTestbitHSpec>;
#[doc = "Register `TSADC_TESTBIT_H` writer"]
pub type W = crate::W<TsadcTestbitHSpec>;
#[doc = "Field `TSADC_TESTBIT_H` reader - tsadc test bit control"]
pub type TsadcTestbitHR = crate::BitReader;
#[doc = "Field `TSADC_TESTBIT_H` writer - tsadc test bit control"]
pub type TsadcTestbitHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - tsadc test bit control"]
    #[inline(always)]
    pub fn tsadc_testbit_h(&self) -> TsadcTestbitHR {
        TsadcTestbitHR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - tsadc test bit control"]
    #[inline(always)]
    #[must_use]
    pub fn tsadc_testbit_h(&mut self) -> TsadcTestbitHW<TsadcTestbitHSpec> {
        TsadcTestbitHW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<TsadcTestbitHSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "tsadc test bit control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsadc_testbit_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsadc_testbit_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsadcTestbitHSpec;
impl crate::RegisterSpec for TsadcTestbitHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsadc_testbit_h::R`](R) reader structure"]
impl crate::Readable for TsadcTestbitHSpec {}
#[doc = "`write(|w| ..)` method takes [`tsadc_testbit_h::W`](W) writer structure"]
impl crate::Writable for TsadcTestbitHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSADC_TESTBIT_H to value 0"]
impl crate::Resettable for TsadcTestbitHSpec {
    const RESET_VALUE: u32 = 0;
}
