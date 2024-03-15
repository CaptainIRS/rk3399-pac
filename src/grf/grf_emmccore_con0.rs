#[doc = "Register `GRF_EMMCCORE_CON0` reader"]
pub type R = crate::R<GrfEmmccoreCon0Spec>;
#[doc = "Register `GRF_EMMCCORE_CON0` writer"]
pub type W = crate::W<GrfEmmccoreCon0Spec>;
#[doc = "Field `EMMCCORE_CON0` reader - emmc controller control register 0"]
pub type EmmccoreCon0R = crate::FieldReader<u16>;
#[doc = "Field `EMMCCORE_CON0` writer - emmc controller control register 0"]
pub type EmmccoreCon0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - emmc controller control register 0"]
    #[inline(always)]
    pub fn emmccore_con0(&self) -> EmmccoreCon0R {
        EmmccoreCon0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - emmc controller control register 0"]
    #[inline(always)]
    #[must_use]
    pub fn emmccore_con0(&mut self) -> EmmccoreCon0W<GrfEmmccoreCon0Spec> {
        EmmccoreCon0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfEmmccoreCon0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "emmc core control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmccore_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmccore_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfEmmccoreCon0Spec;
impl crate::RegisterSpec for GrfEmmccoreCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_emmccore_con0::R`](R) reader structure"]
impl crate::Readable for GrfEmmccoreCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_emmccore_con0::W`](W) writer structure"]
impl crate::Writable for GrfEmmccoreCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_EMMCCORE_CON0 to value 0"]
impl crate::Resettable for GrfEmmccoreCon0Spec {
    const RESET_VALUE: u32 = 0;
}
