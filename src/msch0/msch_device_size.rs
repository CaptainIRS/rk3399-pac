#[doc = "Register `MSCH_DeviceSize` reader"]
pub type R = crate::R<MschDeviceSizeSpec>;
#[doc = "Register `MSCH_DeviceSize` writer"]
pub type W = crate::W<MschDeviceSizeSpec>;
#[doc = "Field `RANK0` reader - Rank0 size."]
pub type Rank0R = crate::FieldReader;
#[doc = "Field `RANK0` writer - Rank0 size."]
pub type Rank0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RANK1` reader - Rank0 size."]
pub type Rank1R = crate::FieldReader;
#[doc = "Field `RANK1` writer - Rank0 size."]
pub type Rank1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Rank0 size."]
    #[inline(always)]
    pub fn rank0(&self) -> Rank0R {
        Rank0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rank0 size."]
    #[inline(always)]
    pub fn rank1(&self) -> Rank1R {
        Rank1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Rank0 size."]
    #[inline(always)]
    #[must_use]
    pub fn rank0(&mut self) -> Rank0W<MschDeviceSizeSpec> {
        Rank0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Rank0 size."]
    #[inline(always)]
    #[must_use]
    pub fn rank1(&mut self) -> Rank1W<MschDeviceSizeSpec> {
        Rank1W::new(self, 8)
    }
}
#[doc = "ddr configuration sizes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msch_device_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msch_device_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MschDeviceSizeSpec;
impl crate::RegisterSpec for MschDeviceSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msch_device_size::R`](R) reader structure"]
impl crate::Readable for MschDeviceSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`msch_device_size::W`](W) writer structure"]
impl crate::Writable for MschDeviceSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSCH_DeviceSize to value 0"]
impl crate::Resettable for MschDeviceSizeSpec {
    const RESET_VALUE: u32 = 0;
}
