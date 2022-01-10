#[doc = "Register `SMHC_TBBCNT` reader"]
pub struct R(crate::R<SMHC_TBBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_TBBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_TBBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_TBBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_TBBCNT` writer"]
pub struct W(crate::W<SMHC_TBBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_TBBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMHC_TBBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_TBBCNT_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transferred Byte Count between Host Memory and Internal FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_tbbcnt](index.html) module"]
pub struct SMHC_TBBCNT_SPEC;
impl crate::RegisterSpec for SMHC_TBBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_tbbcnt::R](R) reader structure"]
impl crate::Readable for SMHC_TBBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_tbbcnt::W](W) writer structure"]
impl crate::Writable for SMHC_TBBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_TBBCNT to value 0"]
impl crate::Resettable for SMHC_TBBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
