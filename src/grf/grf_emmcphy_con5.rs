#[doc = "Register `GRF_EMMCPHY_CON5` reader"]
pub type R = crate::R<GrfEmmcphyCon5Spec>;
#[doc = "Register `GRF_EMMCPHY_CON5` writer"]
pub type W = crate::W<GrfEmmcphyCon5Spec>;
#[doc = "Field `EMMCPHY_CON5` reader - emmc phy control register 5"]
pub type EmmcphyCon5R = crate::FieldReader<u16>;
#[doc = "Field `EMMCPHY_CON5` writer - emmc phy control register 5"]
pub type EmmcphyCon5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - emmc phy control register 5"]
    #[inline(always)]
    pub fn emmcphy_con5(&self) -> EmmcphyCon5R {
        EmmcphyCon5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - emmc phy control register 5"]
    #[inline(always)]
    #[must_use]
    pub fn emmcphy_con5(&mut self) -> EmmcphyCon5W<GrfEmmcphyCon5Spec> {
        EmmcphyCon5W::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfEmmcphyCon5Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "emmc phy control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_emmcphy_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_emmcphy_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfEmmcphyCon5Spec;
impl crate::RegisterSpec for GrfEmmcphyCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_emmcphy_con5::R`](R) reader structure"]
impl crate::Readable for GrfEmmcphyCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_emmcphy_con5::W`](W) writer structure"]
impl crate::Writable for GrfEmmcphyCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_EMMCPHY_CON5 to value 0"]
impl crate::Resettable for GrfEmmcphyCon5Spec {
    const RESET_VALUE: u32 = 0;
}
