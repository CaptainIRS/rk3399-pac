#[doc = "Register `QOS_ExtControl` reader"]
pub type R = crate::R<QosExtControlSpec>;
#[doc = "Register `QOS_ExtControl` writer"]
pub type W = crate::W<QosExtControlSpec>;
#[doc = "Field `SOCKETQOSEN` reader - n/a"]
pub type SocketqosenR = crate::BitReader;
#[doc = "Field `SOCKETQOSEN` writer - n/a"]
pub type SocketqosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTHREN` reader - n/a"]
pub type ExtthrenR = crate::BitReader;
#[doc = "Field `EXTTHREN` writer - n/a"]
pub type ExtthrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTCLKEN` reader - n/a"]
pub type IntclkenR = crate::BitReader;
#[doc = "Field `INTCLKEN` writer - n/a"]
pub type IntclkenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - n/a"]
    #[inline(always)]
    pub fn socketqosen(&self) -> SocketqosenR {
        SocketqosenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - n/a"]
    #[inline(always)]
    pub fn extthren(&self) -> ExtthrenR {
        ExtthrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - n/a"]
    #[inline(always)]
    pub fn intclken(&self) -> IntclkenR {
        IntclkenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - n/a"]
    #[inline(always)]
    #[must_use]
    pub fn socketqosen(&mut self) -> SocketqosenW<QosExtControlSpec> {
        SocketqosenW::new(self, 0)
    }
    #[doc = "Bit 1 - n/a"]
    #[inline(always)]
    #[must_use]
    pub fn extthren(&mut self) -> ExtthrenW<QosExtControlSpec> {
        ExtthrenW::new(self, 1)
    }
    #[doc = "Bit 2 - n/a"]
    #[inline(always)]
    #[must_use]
    pub fn intclken(&mut self) -> IntclkenW<QosExtControlSpec> {
        IntclkenW::new(self, 2)
    }
}
#[doc = "External inputs control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qos_ext_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qos_ext_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QosExtControlSpec;
impl crate::RegisterSpec for QosExtControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qos_ext_control::R`](R) reader structure"]
impl crate::Readable for QosExtControlSpec {}
#[doc = "`write(|w| ..)` method takes [`qos_ext_control::W`](W) writer structure"]
impl crate::Writable for QosExtControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QOS_ExtControl to value 0"]
impl crate::Resettable for QosExtControlSpec {
    const RESET_VALUE: u32 = 0;
}
