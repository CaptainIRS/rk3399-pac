#[doc = "Register `GRF_GPIO3A_E01` reader"]
pub type R = crate::R<GrfGpio3aE01Spec>;
#[doc = "Register `GRF_GPIO3A_E01` writer"]
pub type W = crate::W<GrfGpio3aE01Spec>;
#[doc = "Field `GPIO3A0_E` reader - GPIO3A0 drive strength control bit0 to bit2"]
pub type Gpio3a0ER = crate::FieldReader;
#[doc = "Field `GPIO3A0_E` writer - GPIO3A0 drive strength control bit0 to bit2"]
pub type Gpio3a0EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3A1_E` reader - GPIO3A1 drive strength control bit0 to bit2"]
pub type Gpio3a1ER = crate::FieldReader;
#[doc = "Field `GPIO3A1_E` writer - GPIO3A1 drive strength control bit0 to bit2"]
pub type Gpio3a1EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3A2_E` reader - GPIO3A2 drive strength control bit0 to bit2"]
pub type Gpio3a2ER = crate::FieldReader;
#[doc = "Field `GPIO3A2_E` writer - GPIO3A2 drive strength control bit0 to bit2"]
pub type Gpio3a2EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3A3_E` reader - GPIO3A3 drive strength control bit0 to bit2"]
pub type Gpio3a3ER = crate::FieldReader;
#[doc = "Field `GPIO3A3_E` writer - GPIO3A3 drive strength control bit0 to bit2"]
pub type Gpio3a3EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3A4_E` reader - GPIO3A4 drive strength control bit0 to bit2"]
pub type Gpio3a4ER = crate::FieldReader;
#[doc = "Field `GPIO3A4_E` writer - GPIO3A4 drive strength control bit0 to bit2"]
pub type Gpio3a4EW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GPIO3A5_E0` reader - GPIO3A5 drive strength control bit0"]
pub type Gpio3a5E0R = crate::BitReader;
#[doc = "Field `GPIO3A5_E0` writer - GPIO3A5 drive strength control bit0"]
pub type Gpio3a5E0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - GPIO3A0 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3a0_e(&self) -> Gpio3a0ER {
        Gpio3a0ER::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - GPIO3A1 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3a1_e(&self) -> Gpio3a1ER {
        Gpio3a1ER::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - GPIO3A2 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3a2_e(&self) -> Gpio3a2ER {
        Gpio3a2ER::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO3A3 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3a3_e(&self) -> Gpio3a3ER {
        Gpio3a3ER::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - GPIO3A4 drive strength control bit0 to bit2"]
    #[inline(always)]
    pub fn gpio3a4_e(&self) -> Gpio3a4ER {
        Gpio3a4ER::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - GPIO3A5 drive strength control bit0"]
    #[inline(always)]
    pub fn gpio3a5_e0(&self) -> Gpio3a5E0R {
        Gpio3a5E0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - GPIO3A0 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a0_e(&mut self) -> Gpio3a0EW<GrfGpio3aE01Spec> {
        Gpio3a0EW::new(self, 0)
    }
    #[doc = "Bits 3:5 - GPIO3A1 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a1_e(&mut self) -> Gpio3a1EW<GrfGpio3aE01Spec> {
        Gpio3a1EW::new(self, 3)
    }
    #[doc = "Bits 6:8 - GPIO3A2 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a2_e(&mut self) -> Gpio3a2EW<GrfGpio3aE01Spec> {
        Gpio3a2EW::new(self, 6)
    }
    #[doc = "Bits 9:11 - GPIO3A3 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a3_e(&mut self) -> Gpio3a3EW<GrfGpio3aE01Spec> {
        Gpio3a3EW::new(self, 9)
    }
    #[doc = "Bits 12:14 - GPIO3A4 drive strength control bit0 to bit2"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a4_e(&mut self) -> Gpio3a4EW<GrfGpio3aE01Spec> {
        Gpio3a4EW::new(self, 12)
    }
    #[doc = "Bit 15 - GPIO3A5 drive strength control bit0"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a5_e0(&mut self) -> Gpio3a5E0W<GrfGpio3aE01Spec> {
        Gpio3a5E0W::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3aE01Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_e01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_e01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3aE01Spec;
impl crate::RegisterSpec for GrfGpio3aE01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3a_e01::R`](R) reader structure"]
impl crate::Readable for GrfGpio3aE01Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3a_e01::W`](W) writer structure"]
impl crate::Writable for GrfGpio3aE01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3A_E01 to value 0"]
impl crate::Resettable for GrfGpio3aE01Spec {
    const RESET_VALUE: u32 = 0;
}
