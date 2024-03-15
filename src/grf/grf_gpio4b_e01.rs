#[doc = "Register `GRF_GPIO4B_E01` reader"]
pub type R = crate::R<GrfGpio4bE01Spec>;
#[doc = "Register `GRF_GPIO4B_E01` writer"]
pub type W = crate::W<GrfGpio4bE01Spec>;
#[doc = "Field `GPIO4B0_E` reader - GPIO4B0 drive strength control bit0 to bit2"]
pub type Gpio4b0ER = crate::FieldReader;
#[doc = "Field `GPIO4B0_E` writer - GPIO4B0 drive strength control bit0 to bit2"]
pub type Gpio4b0EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO4B1_E` reader - GPIO4B1 drive strength control bit0 to bit2"]
pub type Gpio4b1ER = crate::FieldReader;
#[doc = "Field `GPIO4B1_E` writer - GPIO4B1 drive strength control bit0 to bit2"]
pub type Gpio4b1EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO4B2_E` reader - GPIO4B2 drive strength control bit0 to bit2"]
pub type Gpio4b2ER = crate::FieldReader;
#[doc = "Field `GPIO4B2_E` writer - GPIO4B2 drive strength control bit0 to bit2"]
pub type Gpio4b2EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO4B3_E` reader - GPIO4B3 drive strength control bit0 to bit2"]
pub type Gpio4b3ER = crate::FieldReader;
#[doc = "Field `GPIO4B3_E` writer - GPIO4B3 drive strength control bit0 to bit2"]
pub type Gpio4b3EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO4B4_E` reader - GPIO4B4 drive strength control bit0 to bit2"]
pub type Gpio4b4ER = crate::FieldReader;
#[doc = "Field `GPIO4B4_E` writer - GPIO4B4 drive strength control bit0 to bit2"]
pub type Gpio4b4EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO4B5_E0` reader - GPIO4B5 drive strength control bit0"]
pub type Gpio4b5E0R = crate::BitReader;
#[doc = "Field `GPIO4B5_E0` writer - GPIO4B5 drive strength control bit0"]
pub type Gpio4b5E0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - GPIO4B0 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio4b0_e(&self) -> Gpio4b0ER {
        Gpio4b0ER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - GPIO4B1 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio4b1_e(&self) -> Gpio4b1ER {
        Gpio4b1ER::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO4B2 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio4b2_e(&self) -> Gpio4b2ER {
        Gpio4b2ER::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO4B3 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio4b3_e(&self) -> Gpio4b3ER {
        Gpio4b3ER::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - GPIO4B4 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio4b4_e(&self) -> Gpio4b4ER {
        Gpio4b4ER::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - GPIO4B5 drive strength control bit0"]
    #[inline(always)]
    pub fn gpio4b5_e0(&self) -> Gpio4b5E0R {
        Gpio4b5E0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO4B0 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b0_e(&mut self) -> Gpio4b0EW<GrfGpio4bE01Spec> {
        Gpio4b0EW::new(self, 0)
    }
    #[doc = "Bits 3:5 - GPIO4B1 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b1_e(&mut self) -> Gpio4b1EW<GrfGpio4bE01Spec> {
        Gpio4b1EW::new(self, 3)
    }
    #[doc = "Bits 6:8 - GPIO4B2 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b2_e(&mut self) -> Gpio4b2EW<GrfGpio4bE01Spec> {
        Gpio4b2EW::new(self, 6)
    }
    #[doc = "Bits 9:11 - GPIO4B3 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b3_e(&mut self) -> Gpio4b3EW<GrfGpio4bE01Spec> {
        Gpio4b3EW::new(self, 9)
    }
    #[doc = "Bits 12:14 - GPIO4B4 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b4_e(&mut self) -> Gpio4b4EW<GrfGpio4bE01Spec> {
        Gpio4b4EW::new(self, 12)
    }
    #[doc = "Bit 15 - GPIO4B5 drive strength control bit0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4b5_e0(&mut self) -> Gpio4b5E0W<GrfGpio4bE01Spec> {
        Gpio4b5E0W::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4bE01Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4b_e01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4b_e01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4bE01Spec;
impl crate::RegisterSpec for GrfGpio4bE01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4b_e01::R`](R) reader structure"]
impl crate::Readable for GrfGpio4bE01Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4b_e01::W`](W) writer structure"]
impl crate::Writable for GrfGpio4bE01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4B_E01 to value 0x9249"]
impl crate::Resettable for GrfGpio4bE01Spec {
    const RESET_VALUE: u32 = 0x9249;
}
