#[doc = "Register `Priority` reader"]
pub type R = crate::R<PrioritySpec>;
#[doc = "Register `Priority` writer"]
pub type W = crate::W<PrioritySpec>;
#[doc = "Field `P0` reader - In Programmable or Bandwidth Limiter mode, the priority level for\n\nwrite transactions. In Bandwidth Regulator mode, the priority level\n\nwhen the used throughput is above the threshold. In Bandwidth\n\nRegulator mode, P0 should have a value equal or lower than P1."]
pub type P0R = crate::FieldReader;
#[doc = "Field `P0` writer - In Programmable or Bandwidth Limiter mode, the priority level for\n\nwrite transactions. In Bandwidth Regulator mode, the priority level\n\nwhen the used throughput is above the threshold. In Bandwidth\n\nRegulator mode, P0 should have a value equal or lower than P1."]
pub type P0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `P1` reader - In Programmable or Bandwidth Limiter mode, the priority level for\n\nread transactions. In Bandwidth regulator mode, the priority level\n\nwhen the used throughput is below the threshold. In Bandwidth\n\nRegulator mode, P1 should have a value equal or greater than P0."]
pub type P1R = crate::FieldReader;
#[doc = "Field `P1` writer - In Programmable or Bandwidth Limiter mode, the priority level for\n\nread transactions. In Bandwidth regulator mode, the priority level\n\nwhen the used throughput is below the threshold. In Bandwidth\n\nRegulator mode, P1 should have a value equal or greater than P0."]
pub type P1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MARK` reader - Backward compatibility marker when 0."]
pub type MarkR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - In Programmable or Bandwidth Limiter mode, the priority level for\n\nwrite transactions. In Bandwidth Regulator mode, the priority level\n\nwhen the used throughput is above the threshold. In Bandwidth\n\nRegulator mode, P0 should have a value equal or lower than P1."]
    #[inline(always)]
    pub fn p0(&self) -> P0R {
        P0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - In Programmable or Bandwidth Limiter mode, the priority level for\n\nread transactions. In Bandwidth regulator mode, the priority level\n\nwhen the used throughput is below the threshold. In Bandwidth\n\nRegulator mode, P1 should have a value equal or greater than P0."]
    #[inline(always)]
    pub fn p1(&self) -> P1R {
        P1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 31 - Backward compatibility marker when 0."]
    #[inline(always)]
    pub fn mark(&self) -> MarkR {
        MarkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - In Programmable or Bandwidth Limiter mode, the priority level for\n\nwrite transactions. In Bandwidth Regulator mode, the priority level\n\nwhen the used throughput is above the threshold. In Bandwidth\n\nRegulator mode, P0 should have a value equal or lower than P1."]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0W<PrioritySpec> {
        P0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - In Programmable or Bandwidth Limiter mode, the priority level for\n\nread transactions. In Bandwidth regulator mode, the priority level\n\nwhen the used throughput is below the threshold. In Bandwidth\n\nRegulator mode, P1 should have a value equal or greater than P0."]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1W<PrioritySpec> {
        P1W::new(self, 2)
    }
}
#[doc = "Priority register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`priority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PrioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets Priority to value 0x8000_0005"]
impl crate::Resettable for PrioritySpec {
    const RESET_VALUE: u32 = 0x8000_0005;
}
