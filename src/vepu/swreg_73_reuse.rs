#[doc = "Register `SWREG_73_REUSE` reader"]
pub type R = crate::R<Swreg73ReuseSpec>;
#[doc = "Register `SWREG_73_REUSE` writer"]
pub type W = crate::W<Swreg73ReuseSpec>;
#[doc = "Field `CHKQP_7` reader - 7st for delta qp check point\n\n7st for delta qp check point"]
pub type Chkqp7R = crate::FieldReader;
#[doc = "Field `CHKQP_7` writer - 7st for delta qp check point\n\n7st for delta qp check point"]
pub type Chkqp7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_6` reader - 6st for delta qp check point\n\n6st for delta qp check point"]
pub type Chkqp6R = crate::FieldReader;
#[doc = "Field `CHKQP_6` writer - 6st for delta qp check point\n\n6st for delta qp check point"]
pub type Chkqp6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_5` reader - 5st for delta qp check point\n\n5st for delta qp check point"]
pub type Chkqp5R = crate::FieldReader;
#[doc = "Field `CHKQP_5` writer - 5st for delta qp check point\n\n5st for delta qp check point"]
pub type Chkqp5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_4` reader - 4st for delta qp check point\n\n4st for delta qp check point"]
pub type Chkqp4R = crate::FieldReader;
#[doc = "Field `CHKQP_4` writer - 4st for delta qp check point\n\n4st for delta qp check point"]
pub type Chkqp4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_3` reader - 3st for delta qp check point\n\n3st for delta qp check point"]
pub type Chkqp3R = crate::FieldReader;
#[doc = "Field `CHKQP_3` writer - 3st for delta qp check point\n\n3st for delta qp check point"]
pub type Chkqp3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_2` reader - 2st for delta qp check point\n\n2st for delta qp check point"]
pub type Chkqp2R = crate::FieldReader;
#[doc = "Field `CHKQP_2` writer - 2st for delta qp check point\n\n2st for delta qp check point"]
pub type Chkqp2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHKQP_1` reader - 1st for delta qp check point\n\n1st for delta qp check point"]
pub type Chkqp1R = crate::FieldReader;
#[doc = "Field `CHKQP_1` writer - 1st for delta qp check point\n\n1st for delta qp check point"]
pub type Chkqp1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 7st for delta qp check point\n\n7st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_7(&self) -> Chkqp7R {
        Chkqp7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 6st for delta qp check point\n\n6st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_6(&self) -> Chkqp6R {
        Chkqp6R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 5st for delta qp check point\n\n5st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_5(&self) -> Chkqp5R {
        Chkqp5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 4st for delta qp check point\n\n4st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_4(&self) -> Chkqp4R {
        Chkqp4R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 3st for delta qp check point\n\n3st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_3(&self) -> Chkqp3R {
        Chkqp3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 2st for delta qp check point\n\n2st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_2(&self) -> Chkqp2R {
        Chkqp2R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 1st for delta qp check point\n\n1st for delta qp check point"]
    #[inline(always)]
    pub fn chkqp_1(&self) -> Chkqp1R {
        Chkqp1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 7st for delta qp check point\n\n7st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_7(&mut self) -> Chkqp7W<Swreg73ReuseSpec> {
        Chkqp7W::new(self, 0)
    }
    #[doc = "Bits 4:7 - 6st for delta qp check point\n\n6st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_6(&mut self) -> Chkqp6W<Swreg73ReuseSpec> {
        Chkqp6W::new(self, 4)
    }
    #[doc = "Bits 8:11 - 5st for delta qp check point\n\n5st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_5(&mut self) -> Chkqp5W<Swreg73ReuseSpec> {
        Chkqp5W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 4st for delta qp check point\n\n4st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_4(&mut self) -> Chkqp4W<Swreg73ReuseSpec> {
        Chkqp4W::new(self, 12)
    }
    #[doc = "Bits 16:19 - 3st for delta qp check point\n\n3st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_3(&mut self) -> Chkqp3W<Swreg73ReuseSpec> {
        Chkqp3W::new(self, 16)
    }
    #[doc = "Bits 20:23 - 2st for delta qp check point\n\n2st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_2(&mut self) -> Chkqp2W<Swreg73ReuseSpec> {
        Chkqp2W::new(self, 20)
    }
    #[doc = "Bits 24:27 - 1st for delta qp check point\n\n1st for delta qp check point"]
    #[inline(always)]
    #[must_use]
    pub fn chkqp_1(&mut self) -> Chkqp1W<Swreg73ReuseSpec> {
        Chkqp1W::new(self, 24)
    }
}
#[doc = "checkpoint delta QP register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg_73_reuse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg_73_reuse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg73ReuseSpec;
impl crate::RegisterSpec for Swreg73ReuseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg_73_reuse::R`](R) reader structure"]
impl crate::Readable for Swreg73ReuseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg_73_reuse::W`](W) writer structure"]
impl crate::Writable for Swreg73ReuseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG_73_REUSE to value 0"]
impl crate::Resettable for Swreg73ReuseSpec {
    const RESET_VALUE: u32 = 0;
}
