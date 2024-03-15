#[doc = "Register `GRF_GPIO3C_E2` reader"]
pub type R = crate::R<GrfGpio3cE2Spec>;
#[doc = "Register `GRF_GPIO3C_E2` writer"]
pub type W = crate::W<GrfGpio3cE2Spec>;
#[doc = "Field `GPIO3C5_E12` reader - GPIO3C5 drive strength control bit1 and bit2"]
pub type Gpio3c5E12R = crate::FieldReader;
#[doc = "Field `GPIO3C5_E12` writer - GPIO3C5 drive strength control bit1 and bit2"]
pub type Gpio3c5E12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GPIO3C6_E` reader - GPIO3C6 drive strength control bit0 to bit2"]
pub type Gpio3c6ER = crate::FieldReader;
#[doc = "Field `GPIO3C6_E` writer - GPIO3C6 drive strength control bit0 to bit2"]
pub type Gpio3c6EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3C7_E` reader - GPIO3C7 drive strength control bit0 to bit2"]
pub type Gpio3c7ER = crate::FieldReader;
#[doc = "Field `GPIO3C7_E` writer - GPIO3C7 drive strength control bit0 to bit2"]
pub type Gpio3c7EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3C5 drive strength control bit1 and bit2"]
    #[inline(always)]
    pub fn gpio3c5_e12(&self) -> Gpio3c5E12R {
        Gpio3c5E12R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - GPIO3C6 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c6_e(&self) -> Gpio3c6ER {
        Gpio3c6ER::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - GPIO3C7 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3c7_e(&self) -> Gpio3c7ER {
        Gpio3c7ER::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3C5 drive strength control bit1 and bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c5_e12(&mut self) -> Gpio3c5E12W<GrfGpio3cE2Spec> {
        Gpio3c5E12W::new(self, 0)
    }
    #[doc = "Bits 2:4 - GPIO3C6 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c6_e(&mut self) -> Gpio3c6EW<GrfGpio3cE2Spec> {
        Gpio3c6EW::new(self, 2)
    }
    #[doc = "Bits 5:7 - GPIO3C7 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3c7_e(&mut self) -> Gpio3c7EW<GrfGpio3cE2Spec> {
        Gpio3c7EW::new(self, 5)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3cE2Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3c_e2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3c_e2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3cE2Spec;
impl crate::RegisterSpec for GrfGpio3cE2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3c_e2::R`](R) reader structure"]
impl crate::Readable for GrfGpio3cE2Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3c_e2::W`](W) writer structure"]
impl crate::Writable for GrfGpio3cE2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3C_E2 to value 0"]
impl crate::Resettable for GrfGpio3cE2Spec {
    const RESET_VALUE: u32 = 0;
}
